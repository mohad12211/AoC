use std::collections::{HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, e)| (*e == 'S').then_some(x as i64))
                .map(|x| (x, y as i64))
        })
        .unwrap();
    let max_steps = 64;
    let count = bfs(max_steps, start, &map);
    println!("{count}");
}

fn bfs(max_steps: i64, start: (i64, i64), map: &Vec<Vec<char>>) -> usize {
    let width = map[0].len() as i64;
    let height = map.len() as i64;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    let mut end_positions = 0;

    while let Some(((x, y), steps)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        if steps % 2 == max_steps % 2 {
            end_positions += 1;
        }
        visited.insert((x, y));
        if steps == max_steps {
            continue;
        }
        visited.insert((x, y));
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = ((x + dx), (y + dy));
            let (inx, iny) = (nx.rem_euclid(width), ny.rem_euclid(height));
            if map[iny as usize][inx as usize] != '#' {
                queue.push_back(((nx, ny), steps + 1));
            }
        }
    }

    end_positions
}
