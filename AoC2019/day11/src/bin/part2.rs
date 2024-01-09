use std::collections::HashMap;

use day11::intcode::Program;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let mut grid = HashMap::new();
    grid.insert((0, 0), 1);
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, -1);
    let mut program = Program::new(&memory);
    loop {
        let (output, halted) = program.run(vec![grid.get(&(x, y)).copied().unwrap_or(0)]);
        grid.insert((x, y), output[0]);
        if output[1] == 0 {
            (dx, dy) = (dy, -dx);
        } else if output[1] == 1 {
            (dx, dy) = (-dy, dx);
        } else {
            unreachable!()
        }
        (x, y) = (x + dx, y + dy);
        if halted {
            break;
        }
    }

    let start_x = *grid.keys().map(|(x, _)| x).min().unwrap();
    let start_y = *grid.keys().map(|(_, y)| y).min().unwrap();
    let end_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let end_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if grid.get(&(x, y)) == Some(&1) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
