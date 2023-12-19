use std::collections::{HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut adj_list = Vec::new();
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let (left, right) = line.split_once(" <-> ").unwrap();
        adj_list.push(right.split(", ").map(|s| s.parse().unwrap()).collect());
        nodes.insert(left.parse().unwrap());
    }

    let mut graph_count = 0;
    while !nodes.is_empty() {
        graph_count += 1;
        let graph = bfs(&adj_list, *nodes.iter().next().unwrap());
        graph.iter().for_each(|x| {
            nodes.remove(x);
        });
    }
    println!("{graph_count}");
}

fn bfs(adj_list: &Vec<Vec<usize>>, start: usize) -> HashSet<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for &neighbour in &adj_list[node] {
            queue.push_back(neighbour);
        }
    }

    visited
}
