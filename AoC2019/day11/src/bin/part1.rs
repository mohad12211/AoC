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
    println!("{}", grid.len());
}
