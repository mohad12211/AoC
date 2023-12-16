use std::collections::{HashSet, VecDeque};

fn get_char((x, y): (i32, i32), grid: &Vec<Vec<char>>) -> Option<char> {
    if x < 0 || x >= (grid[0].len() as i32) || y < 0 || y >= (grid.len() as i32) {
        return None;
    }
    return Some(grid[y as usize][x as usize]);
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let energized_count = discover(&grid);
    println!("{energized_count}");
}

fn discover(grid: &Vec<Vec<char>>) -> usize {
    let mut energized_points = HashSet::with_capacity(grid[0].len() * grid.len());
    let mut beams = HashSet::with_capacity(grid[0].len() * grid.len());
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 1, 0));

    while let Some((mut x, mut y, dx, dy)) = queue.pop_front() {
        if beams.contains(&(x, y, dx, dy)) {
            continue;
        } else {
            beams.insert((x, y, dx, dy));
        }

        loop {
            if get_char((x, y), grid).is_some() {
                energized_points.insert((x, y));
            }
            match get_char((x, y), grid) {
                None => break,
                Some('.') => {
                    x += dx;
                    y += dy;
                }
                Some('-') => {
                    if dy != 0 {
                        queue.push_back((x - 1, y, -1, 0));
                        queue.push_back((x + 1, y, 1, 0));
                        break;
                    } else {
                        x += dx;
                    }
                }
                Some('|') => {
                    if dx != 0 {
                        queue.push_back((x, y - 1, 0, -1));
                        queue.push_back((x, y + 1, 0, 1));
                        break;
                    } else {
                        y += dy;
                    }
                }
                Some('/') => {
                    queue.push_back((x - dy, y - dx, -dy, -dx));
                    break;
                }
                Some('\\') => {
                    queue.push_back((x + dy, y + dx, dy, dx));
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    energized_points.len()
}
