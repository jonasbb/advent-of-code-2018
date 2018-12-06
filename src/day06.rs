use hashbrown::HashMap;
use misc_utils::{Max, Min};
use rayon::prelude::*;

pub struct Coordinate {
    id: usize,
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let mut iter = l.split(", ");
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            Coordinate { id, x, y }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Coordinate]) -> usize {
    let (min_x, max_x, min_y, max_y) = get_bounding_box(input);

    // Lets work with the assumption, that every region which touches the bounding box escapes into infinity
    // Thus everything at the bounding box must be discarded later
    // https://www.reddit.com/r/adventofcode/comments/a3kr4r/2018_day_6_solutions/eb7axrw/

    // Calculate the full grid
    let size_x = max_x - min_x + 1;
    let size_y = max_y - min_y + 1;

    // grid is a matrix which stores the closest ID or None if there is no closest
    let mut grid: Vec<Option<usize>> = vec![None; (size_x * size_y) as usize];

    let set_cell = |grid: &mut Vec<Option<usize>>, x: i32, y: i32, value: Option<usize>| {
        grid[((x - min_x) + (y - min_y) * size_x) as usize] = value;
    };
    let get_cell = |grid: &Vec<Option<usize>>, x: i32, y: i32| -> Option<usize> {
        grid[((x - min_x) + (y - min_y) * size_x) as usize]
    };

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let c = Coordinate {
                id: usize::max_value(),
                x,
                y,
            };
            let mut closest_dist = Min::with_initial(c.distance(&input[0]));
            let mut closest_id = Some(input[0].id);
            for coord in &input[1..] {
                let d = c.distance(coord);
                let cd = closest_dist.get_min_extreme();
                if d == cd {
                    // found two closests, remove value
                    closest_id = None;
                } else if d < cd {
                    closest_dist.update(d);
                    closest_id = Some(coord.id);
                }
            }
            set_cell(&mut grid, x, y, closest_id);
        }
    }

    let mut area_counter: HashMap<usize, usize> = HashMap::new();
    for id in &grid {
        if let Some(id) = id {
            area_counter.entry(*id).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    // Check bounding box and remove those from the area counter
    // upper edge / lower edge
    for x in min_x..=max_x {
        // upper
        if let Some(id) = get_cell(&grid, x, min_y) {
            area_counter.remove(&id);
        }
        // lower
        if let Some(id) = get_cell(&grid, x, max_y) {
            area_counter.remove(&id);
        }
    }
    // left edge / right edge
    for y in min_y..=max_y {
        // left
        if let Some(id) = get_cell(&grid, min_x, y) {
            area_counter.remove(&id);
        }
        // right
        if let Some(id) = get_cell(&grid, max_x, y) {
            area_counter.remove(&id);
        }
    }

    area_counter.values().cloned().max().unwrap()
}

#[aoc(day6, part1, rayon)]
pub fn solve_part1_rayon(input: &[Coordinate]) -> usize {
    let (min_x, max_x, min_y, max_y) = get_bounding_box(input);

    // Lets work with the assumption, that every region which touches the bounding box escapes into infinity
    // Thus everything at the bounding box must be discarded later
    // https://www.reddit.com/r/adventofcode/comments/a3kr4r/2018_day_6_solutions/eb7axrw/

    // Calculate the full grid
    let size_x = max_x - min_x + 1;

    // grid is a matrix which stores the closest ID or None if there is no closest
    #[allow(clippy::range_plus_one)]
    let grid: Vec<Option<usize>> = (min_y..max_y + 1)
        .into_par_iter()
        .flat_map(|y| (min_x..max_x + 1).into_par_iter().map(move |x| (x, y)))
        .map(|(x, y)| {
            let c = Coordinate {
                id: usize::max_value(),
                x,
                y,
            };
            let mut closest_dist = Min::with_initial(c.distance(&input[0]));
            let mut closest_id = Some(input[0].id);
            for coord in &input[1..] {
                let d = c.distance(coord);
                let cd = closest_dist.get_min_extreme();
                if d == cd {
                    // found two closests, remove value
                    closest_id = None;
                } else if d < cd {
                    closest_dist.update(d);
                    closest_id = Some(coord.id);
                }
            }
            closest_id
        })
        .collect();
    let get_cell = |grid: &Vec<Option<usize>>, x: i32, y: i32| -> Option<usize> {
        grid[((x - min_x) + (y - min_y) * size_x) as usize]
    };

    let mut area_counter: HashMap<usize, usize> = HashMap::new();
    for id in &grid {
        if let Some(id) = id {
            area_counter.entry(*id).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    // Check bounding box and remove those from the area counter
    // upper edge / lower edge
    for x in min_x..=max_x {
        // upper
        if let Some(id) = get_cell(&grid, x, min_y) {
            area_counter.remove(&id);
        }
        // lower
        if let Some(id) = get_cell(&grid, x, max_y) {
            area_counter.remove(&id);
        }
    }
    // left edge / right edge
    for y in min_y..=max_y {
        // left
        if let Some(id) = get_cell(&grid, min_x, y) {
            area_counter.remove(&id);
        }
        // right
        if let Some(id) = get_cell(&grid, max_x, y) {
            area_counter.remove(&id);
        }
    }

    area_counter.values().cloned().max().unwrap()
}

fn get_bounding_box(input: &[Coordinate]) -> (i32, i32, i32, i32) {
    // Determine bounding box for Coordinates
    let mut min_x = Min::new();
    let mut max_x = Max::new();
    let mut min_y = Min::new();
    let mut max_y = Max::new();
    for coord in input {
        min_x.update(coord.x);
        max_x.update(coord.x);
        min_y.update(coord.y);
        max_y.update(coord.y);
    }
    (
        min_x.get_min_extreme(),
        max_x.get_max_extreme(),
        min_y.get_min_extreme(),
        max_y.get_max_extreme(),
    )
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Coordinate]) -> usize {
    const DISTANCE_LIMIT: usize = 10000;

    let (min_x, max_x, min_y, max_y) = get_bounding_box(input);
    // There is an absolute limit of DISTANCE_LIMIT
    // So everything outside the bounding box by an amount larger than DISTANCE_LIMIT/input.len()
    // must have a longer distance
    // "/ input.len()", because the distances to all coordinates are summed together
    let min_x = min_x - (DISTANCE_LIMIT / input.len()) as i32;
    let max_x = max_x + (DISTANCE_LIMIT / input.len()) as i32;
    let min_y = min_y - (DISTANCE_LIMIT / input.len()) as i32;
    let max_y = max_y + (DISTANCE_LIMIT / input.len()) as i32;

    // true if the sum of distances to all points is below DISTANCE_LIMIT
    #[allow(clippy::range_plus_one)]
    (min_y..max_y + 1)
        .into_par_iter()
        .flat_map(|y| (min_x..max_x + 1).into_par_iter().map(move |x| (x, y)))
        .filter(|(x, y)| {
            let c = Coordinate {
                id: usize::max_value(),
                x: *x,
                y: *y,
            };
            input
                .par_iter()
                .map(|coord| c.distance(coord) as usize)
                .sum::<usize>()
                < DISTANCE_LIMIT
        })
        .count()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;

#[test]
fn test_part_1() {
    let processed = generator(TEST_INPUT);
    let res = solve_part1(&processed);
    assert_eq!(res, 17)
}

#[test]
fn test_part_2() {
    let processed = generator(TEST_INPUT);
    let res = solve_part2(&processed);
    assert_eq!(res, 16)
}
