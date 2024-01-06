use std::{ops::RangeBounds, str::FromStr};

// https://github.com/rust-lang/rust/blob/88fc543866c2c48b3b1a32e9d55a4eb77d1dee66/src/test/run-pass/const-binops.rs#L12-L19
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-0,
            "{} is not equal to {} in order of {}",
            *a,
            *b,
            1.0e-0
        );
    }};
}

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Vec3 {
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
                z: components[2],
            })
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    r: Vec3,
    v: Vec3,
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
    fn path_intersection(&self, other: &Hailstone) -> Option<Vec3> {
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
        println!("{}", (v1.x * t1 + r1.x) - (v2.x * t2 + r2.x));
        println!("{}", (v1.y * t1 + r1.y) - (v2.y * t2 + r2.y));
        Some(Vec3 {
            x: v1.x * t1 + r1.x,
            y: v1.y * t1 + r1.y,
            z: 0.0,
        })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let hailstones: Vec<Hailstone> = input.lines().map(|l| l.parse().unwrap()).collect();

    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];
            let intersection = h1.path_intersection(h2);
            if let Some(Vec3 { x, y, z: _ }) = intersection {}
        }
    }
    // println!("{hailstone_intersections}");
}
