#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let values: Vec<_> = input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect();
    let mut freq = 0;
    let mut found_freqs = hashbrown::HashSet::with_capacity(values.len());
    for v in values.into_iter().cycle() {
        freq += v;
        // returns true if value was inserted, i.e., not in the set
        if !found_freqs.insert(freq) {
            return freq;
        }
    }
    unreachable!()
}
