use misc_utils::Min;
use rayon::prelude::*;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let unmatched_units = collapse_polymer(input.trim_end().chars());
    unmatched_units.len()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let input = input.trim_end();

    let mut min_length = Min::new();
    // Test which char is the most beneficial to remove
    for unit in "abcdefghijklmnopqrstuvwxyz".chars() {
        // Filter out this unit from the polymer and collapse the rest
        let len = collapse_polymer(input.chars().filter(|&u| u.to_ascii_lowercase() != unit)).len();
        min_length.update(len);
    }
    min_length.get_min().unwrap()
}

#[aoc(day5, part2, iterator)]
pub fn solve_part2_iterator(input: &str) -> usize {
    let input = input.trim_end();

    // Test which char is the most beneficial to remove
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|unit| {
            // Filter out this unit from the polymer and collapse the rest
            collapse_polymer(input.chars().filter(|&u| u.to_ascii_lowercase() != unit)).len()
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2, rayon)]
pub fn solve_part2_par(input: &str) -> usize {
    let input = input.trim_end();

    // Test which char is the most beneficial to remove
    "abcdefghijklmnopqrstuvwxyz"
        .par_chars()
        .map(|unit| {
            // Filter out this unit from the polymer and collapse the rest
            collapse_polymer(input.chars().filter(|&u| u.to_ascii_lowercase() != unit)).len()
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2, early_collapse)]
pub fn solve_part2_early_collapse(input: &str) -> usize {
    let input = input.trim_end();
    let early_collapse = collapse_polymer(input.chars());

    // Test which char is the most beneficial to remove
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|unit| {
            // Filter out this unit from the polymer and collapse the rest
            collapse_polymer(
                early_collapse
                    .iter()
                    .cloned()
                    .filter(|&u| u.to_ascii_lowercase() != unit),
            )
            .len()
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2, rayon_early_collapse)]
pub fn solve_part2_par_early_collapse(input: &str) -> usize {
    let input = input.trim_end();
    let early_collapse = collapse_polymer(input.chars());

    // Test which char is the most beneficial to remove
    "abcdefghijklmnopqrstuvwxyz"
        .par_chars()
        .map(|unit| {
            // Filter out this unit from the polymer and collapse the rest
            collapse_polymer(
                early_collapse
                    .iter()
                    .cloned()
                    .filter(|&u| u.to_ascii_lowercase() != unit),
            )
            .len()
        })
        .min()
        .unwrap()
}

fn collapse_polymer(polymer: impl IntoIterator<Item = char>) -> Vec<char> {
    let mut unmatched_units = Vec::new();
    for unit in polymer {
        if let Some(&last_unit) = unmatched_units.last() {
            if do_units_match(unit, last_unit) {
                // remove the last unit, as it was matched with the current one
                unmatched_units.pop();
            } else {
                // units do not match, append to list of unmatched
                unmatched_units.push(unit);
            }
        } else {
            // units do not match, append to list of unmatched
            unmatched_units.push(unit);
        }
    }
    unmatched_units
}

fn do_units_match(unit_a: char, unit_b: char) -> bool {
    // One is uppercase while the other one is lowercase
    // AND they are they same kind of unit (same letter)
    unit_a.is_uppercase() ^ unit_b.is_uppercase()
        && unit_a.to_ascii_lowercase() == unit_b.to_ascii_lowercase()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"dabAcCaCBAcCcaDA
"#;

#[test]
fn test_part_1_a() {
    let res = solve_part1("aabAAB");
    assert_eq!(res, 6);
}

#[test]
fn test_part_1_b() {
    let res = solve_part1(TEST_INPUT);
    assert_eq!(res, 10);
}

#[test]
fn test_part_2() {
    let res = solve_part2(TEST_INPUT);
    assert_eq!(res, 4);
}
