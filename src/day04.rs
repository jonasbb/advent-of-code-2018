use hashbrown::HashMap;

#[aoc(day4, part1, hashmap)]
pub fn solve_part1(input: &str) -> u32 {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();

    const PREFIX: &str = "[1518-11-05 00:55] ";
    let mut current_guard = 0;
    let mut current_sleep_start = 0;
    let mut minutes_sleep_per_guard: HashMap<u16, u32> = HashMap::new();
    let mut asleep_per_guard_per_minute: HashMap<(u16, u8), u32> = HashMap::new();
    for line in &lines {
        let mut parts = line[PREFIX.len()..].split(' ');
        match parts.next().unwrap() {
            "Guard" => {
                // skip the "#"
                current_guard = parts.next().unwrap()[1..].parse::<u16>().unwrap()
            }
            "falls" => current_sleep_start = parse_minutes(&line),
            "wakes" => {
                let end = parse_minutes(&line);
                let sleep = u32::from(end - current_sleep_start);
                // count total minutes per guard
                minutes_sleep_per_guard
                    .entry(current_guard)
                    .and_modify(|e| *e += sleep)
                    .or_insert(sleep);

                // count for each guard and each minute
                for minute in current_sleep_start..end {
                    asleep_per_guard_per_minute
                        .entry((current_guard, minute))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
            _ => unimplemented!(),
        }
    }

    // find guard with most sleep
    let (guard, _) = minutes_sleep_per_guard
        .into_iter()
        .max_by_key(|(_guard, minutes)| *minutes)
        .unwrap();
    let (minute, _) = (0..60)
        .map(|min| {
            (
                min,
                *asleep_per_guard_per_minute.get(&(guard, min)).unwrap_or(&0),
            )
        })
        .max_by_key(|(_min, count)| *count)
        .unwrap();

    u32::from(guard) * u32::from(minute)
}

#[aoc(day4, part1, vec)]
pub fn solve_part1_vec(input: &str) -> u32 {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();

    const PREFIX: &str = "[1518-11-05 00:55] ";
    let mut current_guard = 0;
    let mut current_sleep_start = 0;
    let mut minutes_sleep_per_guard: HashMap<u16, (u32, Vec<u32>)> = HashMap::new();
    for line in &lines {
        let mut parts = line[PREFIX.len()..].split(' ');
        match parts.next().unwrap() {
            "Guard" => {
                // skip the "#"
                current_guard = parts.next().unwrap()[1..].parse::<u16>().unwrap()
            }
            "falls" => current_sleep_start = parse_minutes(&line),
            "wakes" => {
                let end = parse_minutes(&line);
                let sleep = u32::from(end - current_sleep_start);
                // count total minutes per guard
                let mut entry = minutes_sleep_per_guard
                    .entry(current_guard)
                    .or_insert_with(|| (0, vec![0; 60]));

                entry.0 += sleep;
                // count for each guard and each minute
                for minute in current_sleep_start..end {
                    (entry.1)[minute as usize] += 1;
                }
            }
            _ => unimplemented!(),
        }
    }

    // find guard with most sleep
    let (guard, (_total_count, count_per_minute)) = minutes_sleep_per_guard
        .into_iter()
        .max_by_key(|(_guard, (total_minutes, _per_minute))| *total_minutes)
        .unwrap();
    let (minute, _count) = count_per_minute
        .into_iter()
        .enumerate()
        .max_by_key(|(_index, count)| *count)
        .unwrap();

    u32::from(guard) * minute as u32
}

#[aoc(day4, part2, hashmap)]
pub fn solve_part2(input: &str) -> u32 {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();

    const PREFIX: &str = "[1518-11-05 00:55] ";
    let mut current_guard = 0;
    let mut current_sleep_start = 0;
    let mut asleep_per_guard_per_minute: HashMap<(u16, u8), u32> = HashMap::new();
    for line in &lines {
        let mut parts = line[PREFIX.len()..].split(' ');
        match parts.next().unwrap() {
            "Guard" => {
                // skip the "#"
                current_guard = parts.next().unwrap()[1..].parse::<u16>().unwrap()
            }
            "falls" => current_sleep_start = parse_minutes(&line),
            "wakes" => {
                let end = parse_minutes(&line);
                // count for each guard and each minute
                for minute in current_sleep_start..end {
                    asleep_per_guard_per_minute
                        .entry((current_guard, minute))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
            _ => unimplemented!(),
        }
    }

    let ((guard, minute), _count) = asleep_per_guard_per_minute
        .into_iter()
        .max_by_key(|((_guard, _minute), count)| *count)
        .unwrap();

    u32::from(guard) * u32::from(minute)
}

#[aoc(day4, part2, vec)]
pub fn solve_part2_vec(input: &str) -> u32 {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();

    const PREFIX: &str = "[1518-11-05 00:55] ";
    let mut current_guard = 0;
    let mut current_sleep_start = 0;
    let mut asleep_per_guard_per_minute: HashMap<u16, Vec<u32>> = HashMap::new();
    for line in &lines {
        let mut parts = line[PREFIX.len()..].split(' ');
        match parts.next().unwrap() {
            "Guard" => {
                // skip the "#"
                current_guard = parts.next().unwrap()[1..].parse::<u16>().unwrap()
            }
            "falls" => current_sleep_start = parse_minutes(&line),
            "wakes" => {
                let end = parse_minutes(&line);
                let mut entry = asleep_per_guard_per_minute
                    .entry(current_guard)
                    .or_insert_with(|| vec![0; 60]);
                // count for each guard and each minute
                for minute in current_sleep_start..end {
                    entry[minute as usize] += 1
                }
            }
            _ => unimplemented!(),
        }
    }

    let (guard, minute, _count) = asleep_per_guard_per_minute
        .into_iter()
        .flat_map(|(guard, count_per_minute)| {
            count_per_minute
                .into_iter()
                .enumerate()
                .map(move |(minute, count)| (guard, minute as u32, count))
        })
        .max_by_key(|(_guard, _minute, count)| *count)
        .unwrap();

    u32::from(guard) * minute
}

fn parse_minutes(line: &str) -> u8 {
    const PREFIX: &str = "[1518-11-05 00:";
    line[PREFIX.len()..PREFIX.len() + 2].parse::<u8>().unwrap()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

#[test]
fn test_part_1() {
    let res = solve_part1(TEST_INPUT);
    assert_eq!(res, 240);
}

#[test]
fn test_part_1_vec() {
    let res = solve_part1_vec(TEST_INPUT);
    assert_eq!(res, 240);
}

#[test]
fn test_part_2() {
    let res = solve_part2(TEST_INPUT);
    assert_eq!(res, 4455);
}

#[test]
fn test_part_2_vec() {
    let res = solve_part2_vec(TEST_INPUT);
    assert_eq!(res, 4455);
}
