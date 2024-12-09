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

    let mut antinodes = HashSet::new();

    for freq in map.keys() {
        let points = &map[freq];
        for &(x1, y1) in points {
            for &(x2, y2) in points {
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                let (dx, dy) = (x2 - x1, y2 - y1);
                antinodes.insert((x2 + dx, y2 + dy));
                antinodes.insert((x1 - dx, y1 - dy));
            }
        }
    }

    let result = antinodes
        .iter()
        .filter(|&&(x, y)| x >= 0 && x < cols && y >= 0 && y < rows)
        .count();

    println!("{result}");
}
