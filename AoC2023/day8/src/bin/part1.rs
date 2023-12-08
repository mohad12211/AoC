use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let (directions, nodes) = input.split_once("\n\n").unwrap();
    let directions: Vec<char> = directions.chars().collect();
    let mut map = HashMap::new();

    for line in nodes.lines() {
        let (node, targets) = line.split_once(" = ").unwrap();
        let (left, right) = targets
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(", "))
            .unwrap();
        map.insert(node, (left, right));
    }

    let mut next_node = "AAA";
    let mut next_dir = 0;
    while next_node != "ZZZ" {
        match directions[next_dir % directions.len()] {
            'L' => {
                next_node = map.get(next_node).unwrap().0;
            }
            'R' => {
                next_node = map.get(next_node).unwrap().1;
            }
            _ => unreachable!(),
        }
        next_dir += 1;
    }

    println!("{next_dir}");
}
