use std::collections::{HashSet, VecDeque};
const INPUT: u32 = 1362;

fn is_wall((x, y): (u32, u32), b: u32) -> bool {
    let a = (x * x + 3 * x + 2 * x * y + y + y * y) + b;
    a.count_ones() % 2 != 0
}

fn get_neighbors((x, y): (u32, u32)) -> Vec<(u32, u32)> {
    let mut neighbors = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    neighbors
        .into_iter()
        .filter(|n| !is_wall(*n, INPUT))
        .collect()
}

fn bfs(start: (u32, u32)) -> Option<u32> {
    let mut queue: VecDeque<((u32, u32), u32)> = VecDeque::new();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    queue.push_back((start, 0));

    while let Some((vertex, steps)) = queue.pop_front() {
        if visited.contains(&vertex) {
            continue;
        }
        visited.insert(vertex);
        if steps == 50 {
            continue;
        }
        for current_neighbour in get_neighbors(vertex) {
            queue.push_back((current_neighbour, steps + 1));
        }
    }

    Some(visited.len() as u32)
}

fn main() {
    if let Some(count) = bfs((1, 1)) {
        println!("{:?}", count);
    } else {
        println!("No path found.");
    }
}
