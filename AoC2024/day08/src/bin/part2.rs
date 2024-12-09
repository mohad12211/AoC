use std::collections::{HashMap, HashSet};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &point) in row.iter().enumerate() {
            if point != '.' {
                map.entry(point)
                    .and_modify(|points| points.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        }
    }

    let mut lines = HashSet::new();

    for freq in map.keys() {
        let points = &map[freq];
        for &(x1, y1) in points {
            for &(x2, y2) in points {
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                let (dx, dy) = (x2 - x1, y2 - y1);
                lines.insert((x1, y1, dx, dy));
            }
        }
    }

    let mut points = HashSet::new();

    for (x1, y1, dx, dy) in lines {
        let mut k = 0;
        loop {
            let (nx, ny) = (x1 + k * dx, y1 + k * dy);
            if !(nx >= 0 && nx < cols && ny >= 0 && ny < rows) {
                break;
            }
            points.insert((nx, ny));
            k += 1;
        }
    }

    println!("{}", points.len());
}
