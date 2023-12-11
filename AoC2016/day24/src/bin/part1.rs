use std::collections::{HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    grid: Vec<Vec<char>>,
    x: usize,
    y: usize,
    visited: usize,
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
    let point_count = grid
        .iter()
        .map(|r| r.iter())
        .flatten()
        .filter(|c| c.is_ascii_digit())
        .count();
    // println!("{point_count}");

    let mut map = Map {
        grid,
        x: start_x,
        y: start_y,
        visited: 0,
    };

    if let Some(steps) = map.bfs(point_count) {
        println!("{steps}");
    } else {
        println!("No path found");
    }
}

impl Map {
    fn bfs(&mut self, point_count: usize) -> Option<usize> {
        let mut queue: VecDeque<(Map, usize)> = VecDeque::new();
        let mut visited: HashSet<Map> = HashSet::new();
        queue.push_back((self.clone(), 0));

        while let Some((map, steps)) = queue.pop_front() {
            if visited.contains(&map) {
                continue;
            }
            if map.visited == point_count {
                return Some(steps);
            }
            for next_map in self.next_maps(&map) {
                queue.push_back((next_map, steps + 1));
            }
            visited.insert(map);
        }

        None
    }

    fn next_maps(&self, m: &Map) -> Vec<Map> {
        let mut maps = Vec::new();
        // left
        if m.x > 0 && m.grid[m.y][m.x - 1] != '#' {
            let mut grid = m.grid.clone();
            let mut visited = m.visited;
            if m.grid[m.y][m.x - 1].is_ascii_digit() {
                grid[m.y][m.x - 1] = '.';
                visited += 1;
            }
            maps.push(Map {
                grid,
                x: m.x - 1,
                y: m.y,
                visited,
            });
        }
        // right
        if m.x + 1 < m.grid[0].len() && m.grid[m.y][m.x + 1] != '#' {
            let mut grid = m.grid.clone();
            let mut visited = m.visited;
            if m.grid[m.y][m.x + 1].is_ascii_digit() {
                grid[m.y][m.x + 1] = '.';
                visited += 1;
            }
            maps.push(Map {
                grid,
                x: m.x + 1,
                y: m.y,
                visited,
            });
        }
        // up
        if m.y > 0 && m.grid[m.y - 1][m.x] != '#' {
            let mut grid = m.grid.clone();
            let mut visited = m.visited;
            if m.grid[m.y - 1][m.x].is_ascii_digit() {
                grid[m.y - 1][m.x] = '.';
                visited += 1;
            }
            maps.push(Map {
                grid,
                x: m.x,
                y: m.y - 1,
                visited,
            });
        }
        // down
        if m.y + 1 < m.grid.len() && m.grid[m.y + 1][m.x] != '#' {
            let mut grid = m.grid.clone();
            let mut visited = m.visited;
            if m.grid[m.y + 1][m.x].is_ascii_digit() {
                grid[m.y + 1][m.x] = '.';
                visited += 1;
            }
            maps.push(Map {
                grid,
                x: m.x,
                y: m.y + 1,
                visited,
            });
        }

        maps
    }
}
