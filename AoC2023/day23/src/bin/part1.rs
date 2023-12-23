use std::collections::{HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = 1;
    let end = (grid[0].len() - 2) + grid.len() * (grid.len() - 1);
    let max_steps = dfs(start, end as i32, &grid);
    println!("{max_steps}");
}

fn dfs(start: i32, end: i32, grid: &Vec<Vec<char>>) -> usize {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut max_steps = 0;
    let mut stack: VecDeque<(i32, HashSet<i32>)> = VecDeque::new();
    stack.push_front((start, HashSet::new()));

    while let Some((index, mut path)) = stack.pop_front() {
        if index == end {
            max_steps = max_steps.max(path.len());
            continue;
        }

        path.insert(index);

        let (x, y) = (index % width, index / height);
        match grid[y as usize][x as usize] {
            '.' => {
                for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                    if (0..width).contains(&nx)
                        && (0..height).contains(&ny)
                        && grid[ny as usize][nx as usize] != '#'
                        && !path.contains(&(nx + ny * height))
                    {
                        stack.push_front(((nx + ny * height), path.clone()));
                    }
                }
            }
            'v' => {
                if !path.contains(&(index + height)) {
                    stack.push_front(((index + height), path.clone()));
                }
            }
            '>' => {
                if !path.contains(&(index + 1)) {
                    stack.push_front(((index + 1), path.clone()));
                }
            }
            '<' => {
                if !path.contains(&(index - 1)) {
                    stack.push_front(((index - 1), path.clone()));
                }
            }
            '^' => {
                if !path.contains(&(index + height)) {
                    stack.push_front(((index + height), path.clone()));
                }
            }
            _ => unreachable!(),
        }
    }
    max_steps
}
