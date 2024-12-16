use std::collections::{HashSet, VecDeque};

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut visited = HashSet::new();
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            result += bfs(&grid, &mut visited, (x, y));
        }
    }

    println!("{result}");
}

fn bfs(
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    (x0, y0): (usize, usize),
) -> usize {
    if visited.contains(&(x0, y0)) {
        return 0;
    }
    let region = grid[y0][x0];
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = VecDeque::new();
    queue.push_back((x0, y0));

    while let Some(point @ (x, y)) = queue.pop_front() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        area += 1;
        for (dx, dy) in DIRS {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if grid
                .get(ny)
                .and_then(|row| row.get(nx))
                .is_some_and(|&next_point| next_point == region)
            {
                queue.push_back((nx, ny));
            } else {
                perimeter += 1;
            }
        }
    }

    area * perimeter
}
