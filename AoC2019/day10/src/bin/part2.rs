use std::{
    collections::{BTreeSet, HashSet},
    f64::consts::PI,
    hash::Hash,
};

#[derive(Eq, Debug, Clone, Copy)]
struct Direction {
    dx: i64,
    dy: i64,
}

impl Direction {
    fn angle(&self) -> f64 {
        // angle from the negative y-axis according to the grid coordinate system, [0, 2PI]
        f64::atan2(self.dx as f64, -self.dy as f64).rem_euclid(2. * PI)
    }
    fn simplify(&self) -> Direction {
        let gcd = gcd(self.dx.unsigned_abs(), self.dy.unsigned_abs()) as i64;
        Self {
            dx: self.dx / gcd,
            dy: self.dy / gcd,
        }
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.angle().partial_cmp(&other.angle()).unwrap()
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        let simplified_self = self.simplify();
        let simplified_other = other.simplify();
        simplified_self.dx == simplified_other.dx && simplified_self.dy == simplified_other.dy
    }
}

impl Hash for Direction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let simplified = self.simplify();
        simplified.dx.hash(state);
        simplified.dy.hash(state);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len() as i64;
    let height = grid.len() as i64;
    let mut asteriods = Vec::with_capacity(grid.len());
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '#' {
                asteriods.push((x as i64, y as i64));
            }
        }
    }

    let mut max = 0;
    let (mut max_x, mut max_y) = (0, 0);
    for &(x, y) in &asteriods {
        let mut seen = HashSet::with_capacity(asteriods.len());
        for &(ox, oy) in &asteriods {
            if (x, y) == (ox, oy) {
                continue;
            }
            let dir = Direction {
                dx: ox - x,
                dy: oy - y,
            };
            if !seen.contains(&dir) {
                seen.insert(dir.simplify());
            }
        }
        if max < seen.len() {
            max = seen.len();
            max_x = x;
            max_y = y;
        }
    }

    let (x, y) = (max_x, max_y);
    let mut dirs = BTreeSet::new();
    for &(ox, oy) in &asteriods {
        if (x, y) == (ox, oy) {
            continue;
        }
        let dir = Direction {
            dx: ox - x,
            dy: oy - y,
        };
        if !dirs.contains(&dir) {
            dirs.insert(dir.simplify());
        }
    }

    let mut i = 1;
    loop {
        for &Direction { dx, dy } in dirs.iter() {
            let mut nx = x + dx;
            let mut ny = y + dy;
            while nx >= 0 && nx < width && ny >= 0 && ny < height {
                if grid[ny as usize][nx as usize] == '#' {
                    break;
                } else {
                    nx += dx;
                    ny += dy;
                }
            }
            if !(nx >= 0 && nx < width && ny >= 0 && ny < height) {
                continue;
            }
            grid[ny as usize][nx as usize] = '.';
            if i == 200 {
                println!("{}", nx * 100 + ny);
                return;
            }
            i += 1;
        }
    }
}
