use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Rect {
    point: Point,
    width: i32,
    height: i32,
}

impl FromStr for Rect {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(" @ ").ok_or("Expected ` @ `")?;
        let (coordinates, dimensions) = rest.split_once(": ").ok_or("Expected `: ")?;
        let (x, y) = coordinates.split_once(',').ok_or("Expectd `,`")?;
        let (width, height) = dimensions.split_once('x').ok_or("Expected `x`")?;
        Ok(Rect {
            point: Point {
                x: x.parse().map_err(|_| "Expectd i32 x")?,
                y: y.parse().map_err(|_| "Expectd i32 y")?,
            },
            width: width.parse().map_err(|_| "Expected i32 width")?,
            height: height.parse().map_err(|_| "Expected i32 height")?,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let mut map = HashMap::new();
    input
        .lines()
        .map(|l| Rect::from_str(l).unwrap())
        .for_each(|r| {
            for x in r.point.x..r.point.x + r.width {
                for y in r.point.y..r.point.y + r.height {
                    map.entry(Point { x, y })
                        .and_modify(|n| *n += 1)
                        .or_insert(1);
                }
            }
        });
    let overlap_count = map.values().filter(|&&v| v >= 2).count();
    println!("{overlap_count}")
}
