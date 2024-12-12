use std::collections::{HashSet, VecDeque};

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                result += bfs(&grid, (x, y));
            }
        }
    }
    println!("{result}");
}

fn bfs(grid: &[Vec<usize>], start: (usize, usize)) -> usize {
    let mut count = 0;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front(start);

    while let Some(point @ (x, y)) = queue.pop_front() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        if grid[y][x] == 9 {
            count += 1;
            continue;
        }
        for (dx, dy) in DIRS {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if grid
                .get(ny)
                .and_then(|row| row.get(nx))
                .is_some_and(|&next_point| next_point == grid[y][x] + 1)
            {
                queue.push_front((nx, ny));
            }
        }
    }

    count
}
