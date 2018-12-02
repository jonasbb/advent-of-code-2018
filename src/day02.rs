use hashbrown::HashMap;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let contains_iter = input.lines().map(|l| {
        // count how often each char exists
        let mut char_count = HashMap::new();
        for c in l.trim().chars() {
            char_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        // check if it contains any letter two or three times
        let mut contains_two = false;
        let mut contains_three = false;
        for count in char_count.values() {
            match count {
                2 => contains_two = true,
                3 => contains_three = true,
                _ => {}
            }
        }
        (contains_two, contains_three)
    });

    // Count how many IDs have a letter exactly two/three times
    let mut count_twos = 0;
    let mut count_threes = 0;
    for (contains_two, contains_three) in contains_iter {
        if contains_two {
            count_twos += 1;
        }
        if contains_three {
            count_threes += 1;
        }
    }

    count_twos * count_threes
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    let ids: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    // for each ID, check if there is another similar one
    for (i, id) in ids.iter().enumerate().skip(1) {
        for other_id in &ids[0..i] {
            if is_similar_id(id, other_id) {
                return id
                    .iter()
                    .zip(other_id.iter())
                    // only keep identical characters
                    .filter(|(ca, cb)| ca == cb)
                    .map(|(ca, _cb)| ca)
                    .collect();
            }
        }
    }
    unimplemented!()
}

fn is_similar_id(a: &[char], b: &[char]) -> bool {
    let mut diffs = 0;
    for (ca, cb) in a.iter().zip(b.iter()) {
        if ca != cb {
            diffs += 1;
            if diffs > 1 {
                return false;
            }
        }
    }
    // only similar if they differ by exactly 1
    diffs == 1
}
