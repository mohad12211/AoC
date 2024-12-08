use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let (mut x, mut y): (usize, usize) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .unwrap();
    let (mut dx, mut dy): (isize, isize) = (0, -1);
    let mut path = HashSet::new();
    path.insert((x, y));

    while let Some(&p) = grid
        .get(y.wrapping_add_signed(dy))
        .and_then(|row| row.get(x.wrapping_add_signed(dx)))
    {
        if p == '#' {
            (dx, dy) = (-dy, dx);
            continue;
        }
        (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        path.insert((x, y));
    }

    println!("{}", path.len());
}
