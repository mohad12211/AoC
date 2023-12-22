use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn vertical_len(&self) -> usize {
        self.end.z - self.start.z + 1
    }
    fn intersects(&self, other: &Brick) -> bool {
        self.start.x.max(other.start.x) <= self.end.x.min(other.end.x)
            && self.start.y.max(other.start.y) <= self.end.y.min(other.end.y)
    }
    fn can_fall_to(&self, bricks_below: &[Brick]) -> bool {
        for brick in bricks_below {
            if brick.intersects(self) {
                return false;
            }
        }
        true
    }

    fn find_non_removable(&self, bricks_below: &[Brick]) -> Option<usize> {
        let mut index = None;
        for (brick_index, brick) in bricks_below.iter().enumerate() {
            if self.intersects(brick) {
                if index.is_none() {
                    index = Some(brick_index);
                } else {
                    return None;
                }
            }
        }
        index
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| "Expected numberss separatd by ,")?;
        if components.len() != 3 {
            Err("Expected 3 numberss".into())
        } else {
            Ok(Self {
                x: components[0],
                y: components[1],
                z: components[2],
            })
        }
    }
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').ok_or("Expected ~")?;
        let (start, end) = (start.parse()?, end.parse()?);
        Ok(Self { start, end })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut stack: HashMap<usize, Vec<Brick>> = HashMap::new();
    for brick in input.lines().map(|l| Brick::from_str(l).unwrap()) {
        stack
            .entry(brick.end.z)
            .and_modify(|row| row.push(brick))
            .or_insert(vec![brick]);
    }

    let max_z = *stack.keys().max().unwrap();
    for z in 2..=max_z {
        let Some(current_bricks) = stack.get(&z) else {
            continue;
        };
        let mut falling_bricks = Vec::new();
        for (brick_index, brick) in current_bricks.iter().enumerate() {
            let mut steps_down = 0;
            while steps_down + brick.vertical_len() < z {
                let Some(bricks_below) = stack.get(&(z - steps_down - brick.vertical_len())) else {
                    steps_down += 1;
                    continue;
                };
                if !brick.can_fall_to(bricks_below) {
                    break;
                }
                steps_down += 1;
            }

            if steps_down > 0 {
                falling_bricks.push((brick_index, steps_down));
            }
        }

        for &(brick_index, steps_down) in falling_bricks.iter().rev() {
            let mut falling_brick = stack.get_mut(&z).unwrap().swap_remove(brick_index);
            falling_brick.start.z -= steps_down;
            falling_brick.end.z -= steps_down;
            stack
                .entry(z - steps_down)
                .and_modify(|v| v.push(falling_brick))
                .or_insert(vec![falling_brick]);
        }
    }

    let max_z = *stack
        .iter()
        .filter_map(|(z, bricks)| (!bricks.is_empty()).then_some(z))
        .max()
        .unwrap();

    let mut non_removables = HashSet::new();
    for z in (1..=max_z).rev() {
        let current_bricks = &stack[&z];
        for brick in current_bricks {
            if brick.vertical_len() == z {
                continue;
            }
            let bottom_bricks = &stack[&(z - brick.vertical_len())];
            if let Some(non_removable_index) = brick.find_non_removable(bottom_bricks) {
                non_removables.insert(bottom_bricks[non_removable_index]);
            }
        }
    }

    let total_bricks: usize = stack.values().map(|bricks| bricks.len()).sum();
    println!("{}", total_bricks - non_removables.len());
}
