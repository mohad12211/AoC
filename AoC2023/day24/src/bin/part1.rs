use std::{ops::RangeBounds, str::FromStr};

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl FromStr for Vec2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<f64> = s
            .trim()
            .split(", ")
            .map(|s| s.parse::<f64>())
            .collect::<Result<_, _>>()
            .map_err(|e| e.to_string())?;
        if components.len() != 3 {
            Err("Expected 3 components".into())
        } else {
            Ok(Self {
                x: components[0],
                y: components[1],
            })
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    r: Vec2,
    v: Vec2,
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, v) = s.split_once(" @ ").ok_or("Expected @")?;
        let (r, v) = (r.parse()?, v.parse()?);
        Ok(Self { r, v })
    }
}

impl Hailstone {
    fn path_intersection(&self, other: &Hailstone) -> Option<Vec2> {
        let Hailstone { r: r1, v: v1 } = self;
        let Hailstone { r: r2, v: v2 } = other;
        if v2.x * v1.y == v1.x * v2.y {
            return None; // no intersection
        }
        let drx = r2.x - r1.x;
        let dry = r2.y - r1.y;
        let t1 = (v2.x * dry - drx * v2.y) / (v2.x * v1.y - v1.x * v2.y);
        let t2 = (v1.x * dry - drx * v1.y) / (v2.x * v1.y - v1.x * v2.y);

        if t1 < 0.0 || t2 < 0.0 {
            return None; // occures in past
        }

        Some(Vec2 {
            x: v1.x * t1 + r1.x,
            y: v1.y * t1 + r1.y,
        })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let hailstones: Vec<Hailstone> = input.lines().map(|l| l.parse().unwrap()).collect();
    // let range = 7.0..=27.0;
    let range = 200000000000000_f64..=400000000000000_f64;

    let mut hailstone_intersections = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];
            let intersection = h1.path_intersection(h2);
            if let Some(Vec2 { x, y }) = intersection {
                if range.contains(&x) && range.contains(&y) {
                    hailstone_intersections += 1;
                }
            }
        }
    }
    println!("{hailstone_intersections}");
}
