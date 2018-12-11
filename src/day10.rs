use misc_utils::{Max, Min};

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Point> {
    let offset1 = "position=<".len();
    let offset2 = "position=<     7,      0> velocity=<".len();

    input
        .lines()
        .map(|l| {
            let mut coords = l[offset1..offset1 + 14]
                .split(", ")
                .map(|part| part.trim().parse::<isize>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            let mut dir = l[offset2..offset2 + 6]
                .split(", ")
                .map(|part| part.trim().parse::<isize>().unwrap());
            let delta_x = dir.next().unwrap();
            let delta_y = dir.next().unwrap();
            Point {
                position: Coordinate { x, y },
                velocity: Direction { delta_x, delta_y },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    position: Coordinate,
    velocity: Direction,
}

impl Point {
    fn step(&mut self) {
        self.position.move_coordinate(&self.velocity);
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn move_coordinate(&mut self, direction: &Direction) {
        self.x += direction.delta_x;
        self.y += direction.delta_y;
    }
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    delta_x: isize,
    delta_y: isize,
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Point]) -> String {
    let mut input = input.to_vec();
    for seconds in 0.. {
        if let Some(res) = print_current_step(&input) {
            return format!(
                "\n{}\n\nIt would take {} seconds for the message to appear",
                res, seconds
            );
        }
        input.iter_mut().for_each(|p| p.step());
    }
    unreachable!()
}

fn print_current_step(data: &[Point]) -> Option<String> {
    let mut min_x = Min::new();
    let mut max_x = Max::new();
    let mut min_y = Min::new();
    let mut max_y = Max::new();
    for p in data {
        min_x.update(p.position.x);
        max_x.update(p.position.x);
        min_y.update(p.position.y);
        max_y.update(p.position.y);
    }
    let (min_x, max_x, min_y, max_y) = (
        min_x.get_min_extreme(),
        max_x.get_max_extreme(),
        min_y.get_min_extreme(),
        max_y.get_max_extreme(),
    );

    if ((max_y - min_y + 1) as usize) < 20 {
        let mut field: Vec<Vec<char>> =
            vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        for p in data {
            field[(p.position.y - min_y) as usize][(p.position.x - min_x) as usize] = '#';
        }
        let lines: Vec<String> = field.into_iter().map(|x| x.into_iter().collect()).collect();
        Some((&*lines).join("\n").to_string())
    } else {
        None
    }
}

#[cfg(test)]
const TEST_INPUT: &str = r#"position=<     9,      1> velocity=< 0,  2>
position=<     7,      0> velocity=<-1,  0>
position=<     3,     -2> velocity=<-1,  1>
position=<     6,     10> velocity=<-2, -1>
position=<     2,     -4> velocity=< 2,  2>
position=<    -6,     10> velocity=< 2, -2>
position=<     1,      8> velocity=< 1, -1>
position=<     1,      7> velocity=< 1,  0>
position=<    -3,     11> velocity=< 1, -2>
position=<     7,      6> velocity=<-1, -1>
position=<    -2,      3> velocity=< 1,  0>
position=<    -4,      3> velocity=< 2,  0>
position=<    10,     -3> velocity=<-1,  1>
position=<     5,     11> velocity=< 1, -2>
position=<     4,      7> velocity=< 0, -1>
position=<     8,     -2> velocity=< 0,  1>
position=<    15,      0> velocity=<-2,  0>
position=<     1,      6> velocity=< 1,  0>
position=<     8,      9> velocity=< 0, -1>
position=<     3,      3> velocity=<-1,  1>
position=<     0,      5> velocity=< 0, -1>
position=<    -2,      2> velocity=< 2,  0>
position=<     5,     -2> velocity=< 1,  2>
position=<     1,      4> velocity=< 2,  1>
position=<    -2,      7> velocity=< 2, -2>
position=<     3,      6> velocity=<-1, -1>
position=<     5,      0> velocity=< 1,  0>
position=<    -6,      0> velocity=< 2,  0>
position=<     5,      9> velocity=< 1, -2>
position=<    14,      7> velocity=<-2,  0>
position=<    -3,      6> velocity=< 2, -1>
"#;

#[test]
fn test_part_1() {
    let processed = generator(TEST_INPUT);
    solve_part1(&processed);
}
