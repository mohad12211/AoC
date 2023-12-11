use std::collections::{HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    targets: Vec<(usize, usize)>,
    x: usize,
    y: usize,
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '0').map(|x| (x, y)))
        .unwrap();
    grid[start_y][start_x] = '.';

    let mut targets = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char.is_ascii_digit() {
                targets.push((x, y));
            }
        }
    }

    let mut map = Map {
        targets,
        x: start_x,
        y: start_y,
    };

    if let Some(steps) = map.bfs(grid) {
        println!("{steps}");
    } else {
        println!("No path found");
    }
}

impl Map {
    fn bfs(&mut self, grid: Vec<Vec<char>>) -> Option<usize> {
        let mut queue: VecDeque<(Map, usize)> = VecDeque::new();
        let mut visited: HashSet<Map> = HashSet::new();
        let (start_x, start_y) = (self.x, self.y);
        queue.push_back((self.clone(), 0));

        while let Some((map, steps)) = queue.pop_front() {
            if visited.contains(&map) {
                continue;
            }
            if map.targets.is_empty() && map.x == start_x && map.y == start_y {
                return Some(steps);
            }
            for next_map in self.next_maps(&map, &grid) {
                queue.push_back((next_map, steps + 1));
            }
            visited.insert(map);
        }

        None
    }

    fn next_maps(&self, m: &Map, grid: &Vec<Vec<char>>) -> Vec<Map> {
        let mut maps = Vec::with_capacity(4);

        // left
        if m.x > 0 && grid[m.y][m.x - 1] != '#' {
            let mut next_map = Map {
                targets: m.targets.clone(),
                x: m.x - 1,
                y: m.y,
            };
            if let Some(index) = m.targets.iter().position(|&t| t == (m.x - 1, m.y)) {
                next_map.targets.swap_remove(index);
            }
            maps.push(next_map);
        }
        // right
        if m.x + 1 < grid[0].len() && grid[m.y][m.x + 1] != '#' {
            let mut next_map = Map {
                targets: m.targets.clone(),
                x: m.x + 1,
                y: m.y,
            };
            if let Some(index) = m.targets.iter().position(|&t| t == (m.x + 1, m.y)) {
                next_map.targets.swap_remove(index);
            }
            maps.push(next_map);
        }
        // up
        if m.y > 0 && grid[m.y - 1][m.x] != '#' {
            let mut next_map = Map {
                targets: m.targets.clone(),
                x: m.x,
                y: m.y - 1,
            };
            if let Some(index) = m.targets.iter().position(|&t| t == (m.x, m.y - 1)) {
                next_map.targets.swap_remove(index);
            }
            maps.push(next_map);
        }
        // down
        if m.y + 1 < grid.len() && grid[m.y + 1][m.x] != '#' {
            let mut next_map = Map {
                targets: m.targets.clone(),
                x: m.x,
                y: m.y + 1,
            };
            if let Some(index) = m.targets.iter().position(|&t| t == (m.x, m.y + 1)) {
                next_map.targets.swap_remove(index);
            }
            maps.push(next_map);
        }
        maps
    }
}
