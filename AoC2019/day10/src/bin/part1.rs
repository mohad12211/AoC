use std::{collections::HashSet, hash::Hash};

#[derive(Eq)]
struct Direction(i64, i64);

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        let simplified_self = simplify(self);
        let simplified_other = simplify(other);
        simplified_self.0 == simplified_other.0 && simplified_self.1 == simplified_other.1
    }
}

impl Hash for Direction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let simplified = simplify(self);
        simplified.0.hash(state);
        simplified.1.hash(state);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn simplify(dir: &Direction) -> Direction {
    let gcd = gcd(dir.0.unsigned_abs(), dir.1.unsigned_abs()) as i64;
    Direction(dir.0 / gcd, dir.1 / gcd)
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut asteriods = Vec::with_capacity(grid.len());
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '#' {
                asteriods.push((x as i64, y as i64));
            }
        }
    }

    let mut max = 0;
    for &(x, y) in &asteriods {
        let mut seen = HashSet::with_capacity(asteriods.len());
        for &(ox, oy) in &asteriods {
            if (x, y) == (ox, oy) {
                continue;
            }
            let slope = Direction(ox - x, oy - y);
            if !seen.contains(&slope) {
                seen.insert(slope);
            }
        }
        max = max.max(seen.len());
    }
    println!("{max}");
}
