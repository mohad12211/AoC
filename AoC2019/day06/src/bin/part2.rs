use std::collections::{HashMap, HashSet, VecDeque};

fn bfs(start: &str, end: &str, map: &HashMap<&str, Vec<&str>>) -> Option<usize> {
    let mut qeueu = VecDeque::new();
    let mut visited = HashSet::new();
    qeueu.push_back((start, 0));

    while let Some((object, steps)) = qeueu.pop_front() {
        visited.insert(object);
        if object == end {
            return Some(steps - 2);
        }

        for &neighbour in &map[object] {
            if !visited.contains(&neighbour) {
                qeueu.push_back((neighbour, steps + 1));
            }
        }
    }

    None
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut map = HashMap::new();
    for line in input.lines() {
        let (center, object) = line.split_once(')').unwrap();
        map.entry(object).or_insert(vec![]).push(center);
        map.entry(center).or_insert(vec![]).push(object);
    }

    if let Some(steps) = bfs("YOU", "SAN", &map) {
        println!("{steps}");
    } else {
        println!("No path found");
    }
}
