use misc_utils::Max;

const PREFIX1: usize = "Step ".len();
const PREFIX2: usize = "Step Y must be finished before step ".len();

pub struct Instruction {
    step: u8,
    depends_on: u8,
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let depends_on = l.as_bytes()[PREFIX1] - b'A';
            let step = l.as_bytes()[PREFIX2] - b'A';
            Instruction { step, depends_on }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Instruction]) -> String {
    let mut highest_step = Max::new();

    let mut res = String::with_capacity(26);
    let mut depends_on: Vec<Vec<u8>> = vec![vec![]; 26];

    for inst in input {
        highest_step.update(inst.step);
        highest_step.update(inst.depends_on);
        depends_on[inst.step as usize].push(inst.depends_on)
    }
    let mut step_is_done = vec![false; highest_step.get_max_extreme() as usize + 1];

    'outer: while res.len() <= highest_step.get_max_extreme() as usize {
        for i in 0..=highest_step.get_max_extreme() {
            if !step_is_done[i as usize] {
                // check if any dependency is not done yet
                if !depends_on[i as usize]
                    .iter()
                    .any(|ind| !step_is_done[*ind as usize])
                {
                    // either no deps or all done
                    step_is_done[i as usize] = true;
                    res.push((i as u8 + b'A') as char);
                    continue 'outer;
                }
            }
        }
    }

    res
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Instruction]) -> usize {
    solve_part2_impl(input, 15, 60)
}

fn solve_part2_impl(input: &[Instruction], workers: u8, base_step_cost: u8) -> usize {
    let mut highest_step = Max::new();

    let mut depends_on: Vec<Vec<u8>> = vec![vec![]; 26];
    let mut steps_under_work: Vec<u8> = Vec::with_capacity(workers as usize);

    for inst in input {
        highest_step.update(inst.step);
        highest_step.update(inst.depends_on);
        depends_on[inst.step as usize].push(inst.depends_on)
    }
    let mut step_work_units_left: Vec<u8> = (0..=highest_step.get_max_extreme() as usize)
        .map(|i| i as u8 + 1 + base_step_cost)
        .collect();

    let mut time_spent = 0;

    // This work counts the time spent
    while step_work_units_left.iter().any(|work_left| *work_left > 0) {
        // println!(
        //     "{:?}",
        //     steps_under_work
        //         .iter()
        //         .map(|&x| (x + b'A') as char)
        //         .collect::<String>()
        // );
        // println!(
        //     "{:?}",
        //     step_work_units_left
        //         .iter()
        //         .enumerate()
        //         .map(|(i, &work)| format!("{}: {}", (i as u8 + b'A') as char, work))
        //         .collect::<Vec<String>>()
        // );

        time_spent += 1;

        // for each unit of work currently worked on, decrease the amount of work units left
        // Only keep work units with work left
        steps_under_work = steps_under_work
            .into_iter()
            .filter(|step| {
                step_work_units_left[*step as usize] -= 1;
                step_work_units_left[*step as usize] > 0
            })
            .collect();

        'search_for_work: for i in 0..=highest_step.get_max_extreme() {
            // we only need to search for work until this queue is full
            if steps_under_work.len() == workers as usize {
                break 'search_for_work;
            }

            // search for work which can be distributed
            // If it is already distributed, it cannot be distributed again
            if step_work_units_left[i as usize] > 0 && !steps_under_work.contains(&i) {
                // check if any dependency is not done yet
                if !depends_on[i as usize]
                    .iter()
                    .any(|ind| step_work_units_left[*ind as usize] > 0)
                {
                    // either no deps or all done
                    steps_under_work.push(i);
                    continue 'search_for_work;
                }
            }
        }
    }

    time_spent - 1
}

#[cfg(test)]
const TEST_INPUT: &str = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;

#[test]
fn test_part_1() {
    let processed = generator(TEST_INPUT);
    let res = solve_part1(&processed);
    assert_eq!(res, "CABDFE")
}

#[test]
fn test_part_2_impl() {
    let processed = generator(TEST_INPUT);
    let res = solve_part2_impl(&processed, 2, 0);
    assert_eq!(res, 15)
}
