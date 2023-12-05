use std::str::FromStr;

#[derive(Debug, Default)]
struct Range {
    dst_start: i64,
    src_start: i64,
    len: i64,
}

#[derive(Debug)]
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
    fn contains(&self, num: i64) -> bool {
        num >= self.src_start && num < self.src_start + self.len
    }

    fn convert(&self, num: i64) -> i64 {
        self.dst_start - self.src_start + num
    }
}

impl Map {
    fn convert(&self, num: i64) -> i64 {
        self.ranges
            .iter()
            .find(|r| r.contains(num))
            .map(|r| r.convert(num))
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
    let min: i64 = seeds
        .iter()
        .map(|s| {
            let mut num: i64 = *s;
            for map in &maps {
                num = map.convert(num);
            }
            num
        })
        .min()
        .unwrap();

    println!("{min}");
}
