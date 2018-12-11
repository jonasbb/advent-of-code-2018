#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> String {
    let grid_serial_number: isize = input.trim().parse().unwrap();
    let grid = make_fuel_grid(grid_serial_number);
    let ((x, y), _fuel) = find_largest_area(&grid, 3);
    format!("{},{}", x, y)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> String {
    let grid_serial_number: isize = input.trim().parse().unwrap();
    let grid = make_fuel_grid(grid_serial_number);
    let ((x, y), area_size) = find_best_area(&grid);
    format!("{},{},{}", x, y, area_size)
}

fn make_fuel_grid(grid_serial_number: isize) -> Vec<Vec<isize>> {
    // The grid index starts with 1 for all calculations below
    (1..=300)
        .map(|x| -> Vec<isize> {
            (1..=300)
                .map(|y| get_fuel_level(grid_serial_number, x, y))
                .collect()
        })
        .collect()
}

/// Calculate the fuel level for each cell in the grid
///
/// The power level in a given fuel cell can be found through the following process:
///
/// * Find the fuel cell's rack ID, which is its X coordinate plus 10.
/// * Begin with a power level of the rack ID times the Y coordinate.
/// * Increase the power level by the value of the grid serial number (your puzzle input).
/// * Set the power level to itself multiplied by the rack ID.
/// * Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
/// * Subtract 5 from the power level.
///
/// For example, to find the power level of the fuel cell at 3,5 in a grid with serial number 8:
///
///  * The rack ID is 3 + 10 = 13.
///  * The power level starts at 13 * 5 = 65.
///  * Adding the serial number produces 65 + 8 = 73.
///  * Multiplying by the rack ID produces 73 * 13 = 949.
///  * The hundreds digit of 949 is 9.
///  * Subtracting 5 produces 9 - 5 = 4.
///
/// So, the power level of this fuel cell is 4.
fn get_fuel_level(grid_serial_number: isize, x: isize, y: isize) -> isize {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += grid_serial_number;
    power_level *= rack_id;
    get_hundreds(power_level) - 5
}

fn get_hundreds(value: isize) -> isize {
    // `/ 100` "shifts" the value such that the hundresds are now the ones
    // `% 10` now selects the one place
    (value / 100) % 10
}

/// This functions returns the x and y coordinates, as well as, the amount of fuel in that area
fn find_largest_area(grid: &[Vec<isize>], area_size: usize) -> ((usize, usize), isize) {
    // For an area of 3, we need to count x, x+1, x+2, therefore -1 on the max
    let area_size = area_size - 1;
    let mut current_coords = (0, 0);
    let mut current_area_sum = 0;
    for x in 0..(300 - area_size) {
        for y in 0..(300 - area_size) {
            let area_sum: isize = (x..=x + area_size)
                .flat_map(|a| (y..=y + area_size).map(move |b| (a, b)))
                .map(|(a, b)| grid[a][b])
                .sum();
            if area_sum > current_area_sum {
                current_coords = (x + 1, y + 1);
                current_area_sum = area_sum;
            }
        }
    }
    (current_coords, current_area_sum)
}

/// This functions returns the x and y coordinates, as well as, the size of the area
fn find_best_area(grid: &[Vec<isize>]) -> ((usize, usize), usize) {
    let mut current_coords = (0, 0);
    let mut current_area_sum = 0;
    let mut current_area_size = 0;
    for area_size in 1..20 {
        let (coords, area_sum) = find_largest_area(&grid, area_size);
        if area_sum > current_area_sum {
            current_area_sum = area_sum;
            current_area_size = area_size;
            current_coords = coords;
        }
    }
    (current_coords, current_area_size)
}

#[test]
fn test_make_fuel_grid_1() {
    // Fuel cell at  122,79, grid serial number 57: power level -5.
    assert_eq!(make_fuel_grid(57)[122 - 1][79 - 1], -5);
}

#[test]
fn test_make_fuel_grid_2() {
    // Fuel cell at 217,196, grid serial number 39: power level  0.
    assert_eq!(make_fuel_grid(39)[217 - 1][196 - 1], 0);
}

#[test]
fn test_make_fuel_grid_3() {
    // Fuel cell at 101,153, grid serial number 71: power level  4.
    assert_eq!(make_fuel_grid(71)[101 - 1][153 - 1], 4);
}

#[test]
fn test_get_fuel_level() {
    assert_eq!(get_fuel_level(8, 3, 5), 4)
}

#[test]
fn test_find_largest_area_1() {
    // For grid serial number 18, the largest total 3x3 square has a top-left corner of 33,45 (with a total power of 29)
    let grid = make_fuel_grid(18);
    assert_eq!(find_largest_area(&grid, 3), ((33, 45), 29));
}

#[test]
fn test_find_largest_area_2() {
    // For grid serial number 42, the largest 3x3 square's top-left is 21,61 (with a total power of 30)
    let grid = make_fuel_grid(42);
    assert_eq!(find_largest_area(&grid, 3), ((21, 61), 30));
}

#[test]
fn test_find_best_area_1() {
    // For grid serial number 18, the largest total square (with a total power of 113) is 16x16 and has a top-left corner of 90,269, so its identifier is 90,269,16.
    let grid = make_fuel_grid(18);
    assert_eq!(find_best_area(&grid), ((90, 269), 16));
}

#[test]
fn test_find_best_area_2() {
    // For grid serial number 42, the largest total square (with a total power of 119) is 12x12 and has a top-left corner of 232,251, so its identifier is 232,251,12.
    let grid = make_fuel_grid(42);
    assert_eq!(find_best_area(&grid), ((232, 251), 12));
}
