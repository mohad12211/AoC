use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let (mut x, mut y): (usize, usize) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .unwrap();

    let (x0, y0) = (x, y);

    let (mut dx, mut dy): (isize, isize) = (0, -1);
    let mut path = HashSet::new();
    path.insert((x, y));

    while let Some(&p) = grid
        .get(y.wrapping_add_signed(dy))
        .and_then(|row| row.get(x.wrapping_add_signed(dx)))
    {
        if p == '#' {
            (dx, dy) = (-dy, dx);
        }
        (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        path.insert((x, y));
    }

    let mut loop_count = 0;
    path.remove(&(x0, y0));
    for &(px, py) in &path {
        grid[py][px] = '#';
        if has_loop(&grid, x0, y0) {
            loop_count += 1;
        }
        grid[py][px] = '.';
    }
    println!("{loop_count}");
}

fn has_loop(grid: &[Vec<char>], x0: usize, y0: usize) -> bool {
    let (mut x, mut y) = (x0, y0);

    let (mut dx, mut dy): (isize, isize) = (0, -1);
    let mut path = HashSet::new();
    path.insert((x0, y0, dx, dy));

    while let Some(&p) = grid
        .get(y.wrapping_add_signed(dy))
        .and_then(|row| row.get(x.wrapping_add_signed(dx)))
    {
        if p == '#' {
            (dx, dy) = (-dy, dx);
            continue;
        }
        (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if !path.insert((x, y, dx, dy)) {
            return true;
        }
    }
    return false;
}
