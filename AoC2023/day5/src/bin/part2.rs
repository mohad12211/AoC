use std::str::FromStr;

struct Range {
    dst_start: i64,
    src_start: i64,
    len: i64,
}

struct Map {
    ranges: Vec<Range>,
}

impl From<Vec<Range>> for Map {
    fn from(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        Ok(Self {
            dst_start: nums
                .next()
                .ok_or("range should have destination")?
                .parse()
                .map_err(|_| "destination should be a number")?,
            src_start: nums
                .next()
                .ok_or("range should have source")?
                .parse()
                .map_err(|_| "source should be a number")?,
            len: nums
                .next()
                .ok_or("range should have length")?
                .parse()
                .map_err(|_| "length should be a number")?,
        })
    }
}

impl Range {
    fn outputs(&self, num: i64) -> bool {
        num >= self.dst_start && num < self.dst_start + self.len
    }

    fn invert(&self, num: i64) -> i64 {
        self.src_start - self.dst_start + num
    }
}

impl Map {
    fn invert(&self, num: i64) -> i64 {
        self.ranges
            .iter()
            .find(|r| r.outputs(num))
            .map(|r| r.invert(num))
            .unwrap_or(num)
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let seeds: Vec<(i64, i64)> = seeds.chunks(2).map(|l| (l[0], l[1] + l[0])).collect();

    let maps: Vec<Map> = input
        .split("\n\n")
        .skip(1)
        .map(|x| {
            x.lines()
                .filter_map(|l| Range::from_str(l).ok())
                .collect::<Vec<_>>()
        })
        .map(Map::from)
        .collect();

    for i in 0i64.. {
        let mut num: i64 = i;
        for map in maps.iter().rev() {
            num = map.invert(num);
        }
        if seeds.iter().any(|l| num >= l.0 && num < (l.1)) {
            println!("{i}");
            break;
        }
    }
}
