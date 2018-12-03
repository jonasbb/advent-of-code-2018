use hashbrown::HashMap;

pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn iterate_coords(&self) -> impl Iterator<Item = (u32, u32)> {
        let y = self.y;
        let height = self.height;
        (self.x..self.x + self.width).flat_map(move |a| (y..y + height).map(move |b| (a, b)))
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|l| {
            // parse a line like: "#1 @ 55,885: 22x10"
            let mut parts = l.split(' ');
            let id = parts.next().unwrap()[1..].parse::<u32>().unwrap();
            // skip @
            parts.next();
            let coords = parts.next().unwrap();
            let coords = &coords[..coords.len() - 1];
            let x = coords.split(',').next().unwrap().parse::<u32>().unwrap();
            let y = coords.split(',').nth(1).unwrap().parse::<u32>().unwrap();
            let size = parts.next().unwrap();
            let width = size.split('x').next().unwrap().parse::<u32>().unwrap();
            let height = size.split('x').nth(1).unwrap().parse::<u32>().unwrap();
            Claim {
                id,
                x,
                y,
                width,
                height,
            }
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Claim>) -> usize {
    let counts = count_coords(input);
    // count how many overlaps there are, overlap is count of >1
    counts.values().filter(|&&v| v > 1).count()
}

#[allow(clippy::ptr_arg)]
#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Claim>) -> u32 {
    let counts = count_coords(input);

    'claim: for claim in input {
        // try to find the claim which overlaps with nothing else,
        // meaning all counts must be 1
        for (x, y) in claim.iterate_coords() {
            if counts[&(x, y)] > 1 {
                continue 'claim;
            }
        }
        // only seen 1's
        return claim.id;
    }

    unimplemented!()
}

fn count_coords(input: &Vec<Claim>) -> HashMap<(u32, u32), u32> {
    let iter = input.iter().flat_map(|claim| claim.iterate_coords());
    let mut res = HashMap::new();
    for (x, y) in iter {
        res.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
    }
    res
}

#[test]
fn test_part1() {
    let input = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;
    let res = solve_part1(&generator(input));
    assert_eq!(res, 4);
}
