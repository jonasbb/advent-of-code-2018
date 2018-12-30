#[cfg(test)]
use pretty_assertions::assert_eq;
use std::{
    fmt::{self, Display},
    ops::Index,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    carts: Vec<Cart>,
}

impl Board {
    /// Returns the Coordinate of the collision, if any happen
    fn step(&mut self, remove_crashes: bool) -> Option<Coordinate> {
        let mut carts_sorted = self.carts.clone();
        carts_sorted.sort_by_key(|c| c.position);
        let mut new_carts = Vec::new();

        // skip those indizes while iterating, because the carts crashed
        let mut skiplist = Vec::new();

        for (i, cart) in carts_sorted.iter().enumerate() {
            if skiplist.contains(&i) {
                continue;
            }

            let mut cart = *cart;
            cart.r#move(&self);
            // Check Collision
            if let Some((j, _)) = new_carts
                .iter()
                .enumerate()
                .find(|(_, c): &(usize, &Cart)| c.position == cart.position)
            {
                // Check already moved carts
                if !remove_crashes {
                    return Some(cart.position);
                }
                new_carts.remove(j);
            } else if let Some((j, _)) = carts_sorted[i + 1..]
                .iter()
                .enumerate()
                .find(|(_, c): &(usize, &Cart)| c.position == cart.position)
            {
                // Check not moved carts
                if !remove_crashes {
                    return Some(cart.position);
                }
                skiplist.push(i + j + 1);
            } else {
                new_carts.push(cart);
            }
        }
        self.carts = new_carts;
        // No collision as otherwise we already exited
        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (y, row) in self.fields.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let coord = Coordinate { x, y };
                if let Some(cart) = self.carts.iter().find(|c| c.position == coord) {
                    write!(f, "{}", cart)?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<Coordinate> for Board {
    type Output = Field;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        &self.fields[coord.y][coord.x]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Field {
    // No Track
    Empty,
    // Crossing
    NorthEastSouthWest,
    // Straights
    NorthSouth,
    EastWest,
    // Curves
    NorthEast,
    NorthWest,
    EastSouth,
    SouthWest,
}

impl Field {
    fn from_char(c: char, last_field: Field) -> Self {
        match (c, last_field) {
            (' ', _) => Field::Empty,
            ('|', _) | ('^', _) | ('v', _) => Field::NorthSouth,
            ('-', _) | ('<', _) | ('>', _) => Field::EastWest,
            ('+', _) => Field::NorthEastSouthWest,
            // Those are context sensitive

            // This can be EastSouth or NortWest depending on the last field
            // If the last field has an East connection, then this has to be NorthWest
            ('/', Field::EastSouth)
            | ('/', Field::NorthEastSouthWest)
            | ('/', Field::EastWest)
            | ('/', Field::NorthEast) => Field::NorthWest,
            ('/', _) => Field::EastSouth,

            // This can be NorthEast or SouthWest
            // If there is an East connection in the last field, this has to be SouthWest
            ('\\', Field::EastSouth)
            | ('\\', Field::NorthEastSouthWest)
            | ('\\', Field::EastWest)
            | ('\\', Field::NorthEast) => Field::SouthWest,
            ('\\', _) => Field::NorthEast,
            (c, _) => unreachable!(format!("Unknown track character: '{}'", c)),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                Field::Empty => ' ',
                Field::NorthEastSouthWest => '+',
                Field::NorthSouth => '|',
                Field::EastWest => '-',
                Field::NorthEast | Field::SouthWest => '\\',
                Field::NorthWest | Field::EastSouth => '/',
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Coordinate {
    y: usize,
    x: usize,
}

impl Coordinate {
    fn r#move(&mut self, orientation: Orientation) {
        // Move position, then rotate if on a curve
        match orientation {
            Orientation::North => self.y -= 1,
            Orientation::East => self.x += 1,
            Orientation::South => self.y += 1,
            Orientation::West => self.x -= 1,
        };
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Cart {
    position: Coordinate,
    orientation: Orientation,
    next_crossing_turn: NextCrossingTurn,
}

impl Cart {
    fn new(position: Coordinate, orientation: Orientation) -> Self {
        Cart {
            position,
            orientation,
            next_crossing_turn: NextCrossingTurn::new(),
        }
    }

    fn r#move(&mut self, board: &Board) {
        // Move position, then rotate if on a curve
        self.position.r#move(self.orientation);

        // Rotate if on curve
        match (board[self.position], self.orientation) {
            // The current orientation is always opposite to the field orientation,
            // but the output orientation must be one of the field orientations
            (Field::NorthEast, Orientation::South) => self.orientation = Orientation::East,
            (Field::NorthEast, Orientation::West) => self.orientation = Orientation::North,

            (Field::NorthWest, Orientation::East) => self.orientation = Orientation::North,
            (Field::NorthWest, Orientation::South) => self.orientation = Orientation::West,

            (Field::EastSouth, Orientation::North) => self.orientation = Orientation::East,
            (Field::EastSouth, Orientation::West) => self.orientation = Orientation::South,

            (Field::SouthWest, Orientation::North) => self.orientation = Orientation::West,
            (Field::SouthWest, Orientation::East) => self.orientation = Orientation::South,

            // Special crossing rules
            (Field::NorthEastSouthWest, _) => {
                match self.next_crossing_turn {
                    NextCrossingTurn::Left => self.orientation.left(),
                    NextCrossingTurn::Right => self.orientation.right(),
                    // Nothing to do
                    NextCrossingTurn::Straight => {}
                }

                // Move one step1
                self.next_crossing_turn.next();
            }

            // Nothing to do if not on a curve or crossing
            (_, _) => {}
        }
    }
}

impl Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self.orientation {
                Orientation::North => '^',
                Orientation::East => '>',
                Orientation::South => 'v',
                Orientation::West => '<',
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn left(&mut self) {
        *self = match *self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }
    fn right(&mut self) {
        *self = match *self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NextCrossingTurn {
    Left,
    Straight,
    Right,
}

impl NextCrossingTurn {
    fn new() -> Self {
        NextCrossingTurn::Left
    }

    fn next(&mut self) {
        *self = match *self {
            NextCrossingTurn::Left => NextCrossingTurn::Straight,
            NextCrossingTurn::Straight => NextCrossingTurn::Right,
            NextCrossingTurn::Right => NextCrossingTurn::Left,
        }
    }
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Board {
    let mut carts = Vec::new();
    let board: Vec<Vec<Field>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut last_field = Field::Empty;

            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        '^' => carts.push(Cart::new(Coordinate { x, y }, Orientation::North)),
                        '>' => carts.push(Cart::new(Coordinate { x, y }, Orientation::East)),
                        'v' => carts.push(Cart::new(Coordinate { x, y }, Orientation::South)),
                        '<' => carts.push(Cart::new(Coordinate { x, y }, Orientation::West)),
                        // Not a cart
                        _ => {}
                    }

                    last_field = Field::from_char(c, last_field);
                    last_field
                })
                .collect()
        })
        .collect();
    Board {
        fields: board,
        carts,
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(board: &Board) -> Coordinate {
    let mut board: Board = board.clone();
    loop {
        if let Some(coord) = board.step(false) {
            return coord;
        }
    }
}

#[aoc(day13, part2)]
pub fn solve_part2(board: &Board) -> Coordinate {
    let mut board: Board = board.clone();
    loop {
        board.step(true);
        if board.carts.len() == 1 {
            return board.carts[0].position
        }
    }
}

#[cfg(test)]
const TEST_INPUT: &str = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/";

#[cfg(test)]
const TEST_INPUT_STEP_ONE: &str = r"/-->\
|   |  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \->--/
  \------/
";

#[cfg(test)]
const TEST_INPUT_STEP_TWO: &str = r"/---v
|   |  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \-+>-/
  \------/
";

#[cfg(test)]
const TEST_INPUT_STEP_TEN: &str = r"/---\
|   |  /-<--\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
";

#[test]
fn test_part1_step_one() {
    let mut board = generator(TEST_INPUT);
    board.step(false);
    assert_eq!(board.to_string(), TEST_INPUT_STEP_ONE);
}

#[test]
fn test_part1_step_two() {
    let mut board = generator(TEST_INPUT);
    println!("{:?}", board.fields[0]);
    board.step(false);
    board.step(false);
    assert_eq!(board.to_string(), TEST_INPUT_STEP_TWO);
}

#[test]
fn test_part1_step_ten() {
    let mut board = generator(TEST_INPUT);
    println!("{:?}", board.fields[0]);
    for _ in 0..10 {
        board.step(false);
    }
    assert_eq!(board.to_string(), TEST_INPUT_STEP_TEN);
}

#[test]
fn test_part1() {
    let board = generator(TEST_INPUT);
    let res = solve_part1(&board);
    assert_eq!(res, Coordinate { x: 7, y: 3 });
}

#[test]
fn test_part1_self() {
    let board = generator(">--<");
    let res = solve_part1(&board);
    assert_eq!(res, Coordinate { x: 2, y: 0 });
}

#[cfg(test)]
const TEST_INPUT_PART_2_INPUT: &str = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
";

#[cfg(test)]
const TEST_INPUT_PART_2_STEP_ONE: &str = r"/---\
|   |
| v-+-\
| | | |
\-+-/ |
  |   |
  ^---^
";

#[cfg(test)]
const TEST_INPUT_PART_2_STEP_TWO: &str = r"/---\
|   |
| /-+-\
| v | |
\-+-/ |
  ^   ^
  \---/
";

#[cfg(test)]
const TEST_INPUT_PART_2_STEP_THREE: &str = r"/---\
|   |
| /-+-\
| | | |
\-+-/ ^
  |   |
  \---/
";

#[test]
fn test_part2_step_one() {
    let mut board = generator(TEST_INPUT_PART_2_INPUT);
    println!("{}", board);
    board.step(true);
    println!("{}", board);
    assert_eq!(board.to_string(), TEST_INPUT_PART_2_STEP_ONE);
}

#[test]
fn test_part2_step_two() {
    let mut board = generator(TEST_INPUT_PART_2_INPUT);
    println!("{}", board);
    board.step(true);
    println!("{}", board);
    board.step(true);
    println!("{}", board);
    assert_eq!(board.to_string(), TEST_INPUT_PART_2_STEP_TWO);
}

#[test]
fn test_part2_step_three() {
    let mut board = generator(TEST_INPUT_PART_2_INPUT);
    println!("{}", board);
    board.step(true);
    println!("{}", board);
    board.step(true);
    println!("{}", board);
    board.step(true);
    println!("{}", board);
    assert_eq!(board.to_string(), TEST_INPUT_PART_2_STEP_THREE);
}

#[test]
fn test_part2() {
    let board = generator(TEST_INPUT_PART_2_INPUT);
    let res = solve_part2(&board);
    assert_eq!(res, Coordinate { x: 6, y: 4 });
}
