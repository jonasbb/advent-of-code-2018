#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|l| l.split(' ').map(|x| x.parse().unwrap()))
        .collect()
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.children.iter().map(|child| child.sum()).sum::<usize>()
    }

    fn value(&self) -> usize {
        // The value of a node depends on whether it has child nodes.

        // If a node has **no child nodes**, its value is the sum of its metadata entries. So, the value of
        // node B is 10+11+12=33, and the value of node D is 99.

        // However, if a node **does have child nodes**, the metadata entries become indexes which refer to
        // those child nodes. A metadata entry of 1 refers to the first child node, 2 to the second, 3 to
        // the third, and so on. The value of this node is the sum of the values of the child nodes
        // referenced by the metadata entries. If a referenced child node does not exist, that reference is
        // skipped. A child node can be referenced multiple time and counts each time it is referenced. A
        // metadata entry of 0 does not refer to any child node.

        if self.children.is_empty() {
            self.sum()
        } else {
            self.metadata
                .iter()
                .map(|&idx| {
                    if idx == 0 {
                        0
                    } else if let Some(child) = self.children.get(idx-1) {
                        child.value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    // Each node is specified by two values, first the number of child nodes, then the number of metadata entries
    let root = parse_node(&mut input.iter().cloned());
    root.sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    // Each node is specified by two values, first the number of child nodes, then the number of metadata entries
    let root = parse_node(&mut input.iter().cloned());
    root.value()
}

fn parse_node<I>(input: &mut I) -> Node
where
    I: Iterator<Item = usize>,
{
    let child_count = input.next().unwrap();
    let metadata_count = input.next().unwrap();

    let children: Vec<Node> = (0..child_count).map(|_| parse_node(input)).collect();
    let metadata: Vec<usize> = input.take(metadata_count as usize).collect();

    Node { children, metadata }
}

#[cfg(test)]
const TEST_INPUT: &str = r#"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
"#;

#[test]
fn test_part_1() {
    let processed = generator(TEST_INPUT);
    let res = solve_part1(&processed);
    assert_eq!(res, 138)
}

#[test]
fn test_part_2() {
    let processed = generator(TEST_INPUT);
    let res = solve_part2(&processed);
    assert_eq!(res, 66)
}
