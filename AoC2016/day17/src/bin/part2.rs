use md5::compute;
use std::collections::VecDeque;

const CODE: &str = "rrrbmfta";

fn get_neighbors(path: &str) -> String {
    let hash = format!("{:x}", compute(format!("{CODE}{path}")));
    let mut doors = hash.as_bytes()[0..4].into_iter().map(is_open);
    let mut neighbors = String::new();
    let (x, y) = path_to_point(path);
    if doors.next().unwrap() && y - 1 >= 0 {
        neighbors.push('U');
    }
    if doors.next().unwrap() && y + 1 <= 3 {
        neighbors.push('D');
    }
    if doors.next().unwrap() && x - 1 >= 0 {
        neighbors.push('L');
    }
    if doors.next().unwrap() && x + 1 <= 3 {
        neighbors.push('R');
    }
    neighbors
}

fn bfs() -> usize {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut max_len = 0;
    queue.push_back(String::new());

    while let Some(path) = queue.pop_front() {
        if is_end(&path) {
            max_len = max_len.max(path.len());
            continue;
        }
        for neighbor in get_neighbors(&path).chars() {
            let mut neighbor_path = path.clone();
            neighbor_path.push(neighbor);
            queue.push_back(neighbor_path);
        }
    }

    max_len
}

fn path_to_point(s: &str) -> (i32, i32) {
    s.chars().fold((0, 0), |(x, y), c| match c {
        'U' => (x, y - 1),
        'D' => (x, y + 1),
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        _ => unreachable!(),
    })
}

fn is_end(s: &str) -> bool {
    let (x, y) = path_to_point(s);
    x == 3 && y == 3
}

fn is_open(u: &u8) -> bool {
    (b'b'..=b'f').contains(&u)
}

fn main() {
    let longest = bfs();
    println!("{}", longest);
}
