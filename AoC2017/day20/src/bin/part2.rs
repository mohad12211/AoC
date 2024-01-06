use std::cmp::Ordering;
// https://www.reddit.com/r/adventofcode/comments/7l1766/comment/drio4yp/?utm_source=reddit&utm_medium=web2x&context=3

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn parse(s: &str) -> Self {
        let len = s.len();
        let v3 = s[1..len - 1].split(',').collect::<Vec<_>>();
        Self {
            x: v3[0].parse().unwrap(),
            y: v3[1].parse().unwrap(),
            z: v3[2].parse().unwrap(),
        }
    }

    fn dist(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.dist().cmp(&other.dist()))
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Particle {
    a: Vec3,
    v: Vec3,
    p: Vec3,
}

impl Particle {
    fn parse(s: &str) -> Self {
        let three_vec = s.split(", ").collect::<Vec<_>>();
        Self {
            a: Vec3::parse(three_vec[2].split_once('=').unwrap().1),
            v: Vec3::parse(three_vec[1].split_once('=').unwrap().1),
            p: Vec3::parse(three_vec[0].split_once('=').unwrap().1),
        }
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let particles: Vec<_> = input.lines().map(|l| Particle::parse(l)).collect();
}
