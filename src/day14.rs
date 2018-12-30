#[cfg(test)]
use pretty_assertions::assert_eq;
use std::fmt::{self, Display};

struct State {
    receipies: Vec<u8>,
    first_elf: usize,
    second_elf: usize,
}

impl State {
    fn new() -> Self {
        Self {
            receipies: vec![3, 7],
            first_elf: 0,
            second_elf: 1,
        }
    }

    fn step(&mut self) {
        // Make new receipies
        let r_elf_1 = self.receipies[self.first_elf];
        let r_elf_2 = self.receipies[self.second_elf];
        let new_r = r_elf_1 + r_elf_2;
        let r1 = new_r / 10;
        let r2 = new_r % 10;
        if r1 > 0 {
            self.receipies.push(r1);
        }
        self.receipies.push(r2);

        // Switch receipies of elves
        self.first_elf = (self.first_elf + 1 + r_elf_1 as usize) % self.receipies.len();
        self.second_elf = (self.second_elf + 1 + r_elf_2 as usize) % self.receipies.len();
    }

    fn take_10_after_index(&self, ind: usize) -> String {
        self.receipies
            .iter()
            .skip(ind)
            .take(10)
            .map(|r| format!("{}", r))
            .collect()
    }

    #[allow(clippy::int_plus_one)]
    fn ends_with(&self, needle: &[u8]) -> Option<usize> {
        // Check how many receipies are before the needle

        // Each round adds either one or two receipies, therefore we test two positions
        let len = self.receipies.len();
        if len >= needle.len() + 1 && &self.receipies[len - needle.len() - 1..len - 1] == needle {
            Some(len - needle.len() - 1)
        } else if len >= needle.len() && &self.receipies[len - needle.len()..len] == needle {
            Some(len - needle.len())
        } else {
            None
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (i, r) in self.receipies.iter().enumerate() {
            if i == self.first_elf {
                write!(f, "({})", r)?;
            } else if i == self.second_elf {
                write!(f, "[{}]", r)?;
            } else {
                write!(f, " {} ", r)?;
            }
        }
        writeln!(f)
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> String {
    let input: usize = input.parse().unwrap();
    let mut state = State::new();

    while state.receipies.len() < input + 10 {
        state.step()
    }

    state.take_10_after_index(input)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let receipies: Vec<u8> = input.chars().map(|c| c as u8 - b'0').collect();
    let mut state = State::new();
    loop {
        state.step();
        if let Some(ind) = state.ends_with(&receipies) {
            return ind;
        }
    }
}

#[test]
fn test_part1_5() {
    assert_eq!(solve_part1("5"), "0124515891");
}

#[test]
fn test_part1_9() {
    assert_eq!(solve_part1("9"), "5158916779");
}

#[test]
fn test_part1_18() {
    assert_eq!(solve_part1("18"), "9251071085");
}

#[test]
fn test_part1_2018() {
    assert_eq!(solve_part1("2018"), "5941429882");
}

#[test]
fn test_part2_5() {
    assert_eq!(solve_part2("01245"), 5);
}

#[test]
fn test_part2_9() {
    assert_eq!(solve_part2("51589"), 9);
}

#[test]
fn test_part2_18() {
    assert_eq!(solve_part2("92510"), 18);
}

#[test]
fn test_part2_2018() {
    assert_eq!(solve_part2("59414"), 2018);
}
