use intrusive_collections::{linked_list::CursorMut, LinkedList, LinkedListLink};

#[aoc(day9, part1, vector)]
pub fn solve_part1(_input: &str) -> usize {
    solve_part1_impl(476, 71431)
}

#[aoc(day9, part1, linked_list)]
pub fn solve_part1_linked_list(_input: &str) -> usize {
    solve_part1_impl_linked_list(476, 71431)
}

#[aoc(day9, part2)]
pub fn solve_part2(_input: &str) -> usize {
    solve_part1_impl_linked_list(476, 71431 * 100)
}

fn solve_part1_impl(players: usize, highest_marble_value: usize) -> usize {
    let mut marble_ring = Vec::with_capacity(highest_marble_value + 1);
    let mut current_marble: usize;
    let mut current_player: usize;
    let mut points_per_player: Vec<usize> = vec![0; players];

    let step_player = |player: usize| -> usize { (player + 1) % players };

    // init the board to a reasonable initial size
    marble_ring.push(0);
    marble_ring.push(2);
    marble_ring.push(1);
    current_player = 1;
    current_marble = 1;

    for value in 3..=highest_marble_value {
        // eprintln!(
        //     "[{}] {}",
        //     current_player + 1,
        //     marble_ring
        //         .iter()
        //         .enumerate()
        //         .map(|(i, value)| format!(
        //             "{:>2}{}",
        //             value,
        //             if i == current_marble { '*' } else { ' ' },
        //         ))
        //         .collect::<String>()
        // );

        current_player = step_player(current_player);
        if value % 23 != 0 {
            current_marble = (current_marble + 2) % marble_ring.len();
            marble_ring.insert(current_marble, value);
        } else {
            // Add the extra length to make sure it never underflows
            current_marble = (current_marble + marble_ring.len() - 7) % marble_ring.len();
            points_per_player[current_player] += value;
            points_per_player[current_player] += marble_ring.remove(current_marble);
        }
    }

    // eprintln!(
    //     "[{}] {}",
    //     current_player + 1,
    //     marble_ring
    //         .iter()
    //         .enumerate()
    //         .map(|(i, value)| format!(
    //             "{:>2}{}",
    //             value,
    //             if i == current_marble { '*' } else { ' ' },
    //         ))
    //         .collect::<String>()
    // );

    // for (player, points) in points_per_player.iter().enumerate() {
    //     println!("Player {:>2}: {}", player + 1, points);
    // }

    points_per_player.into_iter().max().unwrap()
}

// A simple struct containing an instrusive link and a value
struct Node {
    link: LinkedListLink,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Box<Self> {
        Box::new(Node {
            link: LinkedListLink::new(),
            value,
        })
    }
}

// The adapter describes how an object can be inserted into an intrusive
// collection. This is automatically generated using a macro.
intrusive_adapter!(NodeAdapter = Box<Node>: Node { link: LinkedListLink });

fn solve_part1_impl_linked_list(players: usize, highest_marble_value: usize) -> usize {
    let mut marble_ring = LinkedList::new(NodeAdapter::new());
    let mut current_player: usize;
    let mut points_per_player: Vec<usize> = vec![0; players];

    let step_player = |player: usize| -> usize { (player + 1) % players };

    // init the board to a reasonable initial size
    marble_ring.push_back(Node::new(0));
    marble_ring.push_back(Node::new(2));
    marble_ring.push_back(Node::new(1));
    current_player = 1;
    // Set a cursor to the Node with value 2
    let mut current_marble = marble_ring.front_mut();
    current_marble.move_next();

    for value in 3..=highest_marble_value {
        current_player = step_player(current_player);
        if value % 23 != 0 {
            move_forward(&mut current_marble, 1);
            current_marble.insert_after(Node::new(value));
            // move onto the newly inserted value
            move_forward(&mut current_marble, 1);
        } else {
            move_backwards(&mut current_marble, 7);
            points_per_player[current_player] += value;
            points_per_player[current_player] += current_marble.remove().unwrap().value;
            // It could now be that we are on the null element after the remove, step of it if that is the case
            if current_marble.is_null() {
                current_marble.move_next();
            }
        }
    }

    points_per_player.into_iter().max().unwrap()
}

fn move_forward(cursor: &mut CursorMut<NodeAdapter>, steps: usize) {
    for _ in 0..steps {
        cursor.move_next();
        // At the edges we have the special null element which we want to skip
        if cursor.is_null() {
            cursor.move_next();
        }
    }
}

fn move_backwards(cursor: &mut CursorMut<NodeAdapter>, steps: usize) {
    for _ in 0..steps {
        cursor.move_prev();
        // At the edges we have the special null element which we want to skip
        if cursor.is_null() {
            cursor.move_prev();
        }
    }
}

#[test]
fn test_part_1_a() {
    let res = solve_part1_impl(9, 25);
    assert_eq!(res, 32)
}

#[test]
fn test_part_1_a_linked_list() {
    let res = solve_part1_impl_linked_list(9, 25);
    assert_eq!(res, 32)
}

#[test]
fn test_part_1_b() {
    let res = solve_part1_impl(10, 1618);
    assert_eq!(res, 8317)
}

#[test]
fn test_part_1_c() {
    let res = solve_part1_impl(13, 7999);
    assert_eq!(res, 146_373)
}

#[test]
fn test_part_1_d() {
    let res = solve_part1_impl(17, 1104);
    assert_eq!(res, 2764)
}

#[test]
fn test_part_1_e() {
    let res = solve_part1_impl(21, 6111);
    assert_eq!(res, 54718)
}

#[test]
fn test_part_1_f() {
    let res = solve_part1_impl(30, 5807);
    assert_eq!(res, 37305)
}
