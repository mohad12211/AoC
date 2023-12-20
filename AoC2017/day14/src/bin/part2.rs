use std::collections::{HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let key = input.trim();
    let mut points = HashSet::new();

    for y in 0..128 {
        let bitmap = u128::from_str_radix(&hash(&format!("{key}-{y}")), 16).unwrap();
        for x in 0..128 {
            if check(bitmap, x) {
                points.insert((x, y));
            }
        }
    }

    let mut regions = 0;
    while !points.is_empty() {
        regions += 1;
        let start = points.iter().next().unwrap();
        let region = bfs(&points, *start);
        for point in region {
            points.remove(&point);
        }
    }

    println!("{regions}");
}

fn bfs(points: &HashSet<(i32, i32)>, start: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            if (0..128).contains(&nx) && (0..128).contains(&ny) && points.contains(&(nx, ny)) {
                queue.push_back((nx, ny));
            }
        }
    }

    visited
}

fn hash(input: &str) -> String {
    let size = 256;
    let mut nums: Vec<_> = (0..size).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in input.as_bytes().iter().copied().chain([17, 31, 73, 47, 23]) {
            let length = length as usize;
            let start = current_position;
            let end = current_position + length;

            if end > size {
                let higher_slice = &nums[start..];
                let lower_slice = &nums[..(end % size)];
                let mut full_slice = [higher_slice, lower_slice].concat();
                full_slice.reverse();
                nums[start..].copy_from_slice(&full_slice[..(size - start)]);
                nums[..(end % size)].copy_from_slice(&full_slice[(size - start)..]);
            } else {
                nums[start..end].reverse();
            }

            current_position = (current_position + length + skip_size) % size;
            skip_size += 1;
        }
    }

    nums.chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|acc, n| acc ^ n).unwrap())
        .map(|x| format!("{:02x}", x))
        .collect()
}

// check if ith bit is set (0-based from left)
fn check(n: u128, i: i32) -> bool {
    ((n) & (1 << (127 - i))) != 0
}
