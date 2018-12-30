const PREFIX: &str = "initial state: ";
// 20 generations, each allows to jump at most 2 places to the left
const OFFSET: usize = 2 * 20;

pub struct Rule {
    pattern: Vec<bool>,
    new_state: bool,
}

fn str_to_iter_bool<'a>(s: &'a str) -> impl Iterator<Item = bool> + 'a {
    s.chars().map(|c| c == '#')
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> (Vec<bool>, Vec<Rule>) {
    let mut lines = input.lines();
    // Create OFFSET `false` entries, then followed be the plants
    let plants: Vec<bool> = (0..OFFSET)
        .map(|_| false)
        .chain(str_to_iter_bool(&lines.next().unwrap()[PREFIX.len()..]))
        .chain((0..OFFSET).map(|_| false))
        .collect();

    // skip empty lines
    lines.next();

    let rules: Vec<_> = lines
        .map(|l| Rule {
            pattern: str_to_iter_bool(&l[0..5]).collect(),
            new_state: &l[9..10] == "#",
        })
        .collect();

    (plants, rules)
}

#[aoc(day12, part1)]
pub fn solve_part1((plants, rules): &(Vec<bool>, Vec<Rule>)) -> isize {
    let mut plants = plants.clone();

    for _ in 0..20 {
        // Perform one step
        let new_plants: Vec<bool> = [false, false]
            .iter()
            .cloned()
            .chain(plants.windows(5).map(|window| {
                for rule in rules {
                    if rule.pattern == window {
                        return rule.new_state;
                    }
                }
                unreachable!("There must always be a rule matching the window.")
            }))
            .chain([false, false].iter().cloned())
            .collect();

        assert_eq!(plants.len(), new_plants.len());
        plants = new_plants;
    }

    plants
        .into_iter()
        .enumerate()
        .map(|(i, has_plant)| {
            if has_plant {
                i as isize - OFFSET as isize
            } else {
                0
            }
        })
        .sum()
}
