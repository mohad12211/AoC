use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash)]
struct Claim {
    point: Point,
    width: i32,
    height: i32,
    id: i32,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(" @ ").ok_or("Expected ` @ `")?;
        let (coordinates, dimensions) = rest.split_once(": ").ok_or("Expected `: ")?;
        let (x, y) = coordinates.split_once(',').ok_or("Expectd `,`")?;
        let (width, height) = dimensions.split_once('x').ok_or("Expected `x`")?;
        Ok(Claim {
            point: Point {
                x: x.parse().map_err(|_| "Expectd i32 x")?,
                y: y.parse().map_err(|_| "Expectd i32 y")?,
            },
            width: width.parse().map_err(|_| "Expected i32 width")?,
            height: height.parse().map_err(|_| "Expected i32 height")?,
            id: id
                .strip_prefix('#')
                .ok_or("Expected `#` before id")?
                .parse()
                .map_err(|_| "Expected i32 id")?,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let mut map = HashMap::new();
    let mut untouched = HashSet::new();
    input
        .lines()
        .map(|l| Claim::from_str(l).unwrap())
        .for_each(|c| {
            untouched.insert(c.id);
            for x in c.point.x..c.point.x + c.width {
                for y in c.point.y..c.point.y + c.height {
                    let point = Point { x, y };
                    if let Some(placed_id) = map.get(&point) {
                        untouched.remove(placed_id);
                        untouched.remove(&c.id);
                    }
                    map.insert(point, c.id);
                }
            }
        });
    let untouched_id = untouched.iter().next().unwrap();
    println!("{untouched_id}")
}
