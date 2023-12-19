use std::collections::{HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut adj_list = Vec::new();
    for line in input.lines() {
        let (_, right) = line.split_once(" <-> ").unwrap();
        adj_list.push(right.split(", ").map(|s| s.parse().unwrap()).collect());
    }
    let connected_to_zero = bfs(&adj_list);
    println!("{connected_to_zero}");
}

fn bfs(adj_list: &Vec<Vec<usize>>) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(0);

    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for &neighbour in &adj_list[node] {
            queue.push_back(neighbour);
        }
    }

    visited.len()
}
