fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32 - 1; // ignore extra new line at the end
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let (mut x, mut y): (usize, usize) =
        (grid[0].iter().position(|c| !c.is_whitespace()).unwrap(), 0);
    let (mut dx, mut dy) = (0, 1);
    let mut s = String::new();

    'outer: loop {
        // println!("at: {x}, {y}");
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            let (nx, ny) = (nx as usize, ny as usize);
            match grid[ny][nx] {
                '-' | '|' | '+' => {
                    (x, y) = (nx, ny);
                    continue;
                }
                c if c.is_ascii_alphabetic() => {
                    s.push(c);
                    (x, y) = (nx, ny);
                    continue;
                }
                ' ' => {}
                _ => unreachable!(),
            }
        }

        for (ndx, ndy) in directions {
            if (ndx, ndy) == (-dx, -dy) {
                continue;
            }
            let (nx, ny) = (x as i32 + ndx, y as i32 + ndy);
            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[ny][nx] != ' ' {
                    (dx, dy) = (ndx, ndy);
                    continue 'outer;
                }
            }
        }

        break;
    }

    println!("{s}");
}
