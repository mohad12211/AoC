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

    let periods: Vec<u64> = map
        .iter()
        .filter(|(node, _)| node.ends_with('A'))
        .map(|(&node, _)| {
            let mut next_node = node;
            let mut next_dir = 0;
            while !next_node.ends_with('Z') {
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
            next_dir as u64
        })
        .collect();
    println!("{}", total_lcm(periods).unwrap());
}

fn total_lcm(nums: Vec<u64>) -> Option<u64> {
    nums.into_iter().reduce(lcm)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
