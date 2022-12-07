fn main() {
    let input = include_str!("../inputs/day3.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let first = &line[0..line.len() / 2];
        let second = &line[line.len() / 2..];
        let common = get_common(first, second).expect("should find common char");
        sum += match common.is_ascii_uppercase() {
            true => (common - b'A') as i32 + 1 + 26,
            false => (common - b'a') as i32 + 1,
        };
    }
    println!("{}", sum);
}

fn get_common(first: &str, second: &str) -> Option<u8> {
    let mut array: [bool; 52] = [false; 52];
    for c in first.bytes() {
        let index = match c.is_ascii_lowercase() {
            true => c - b'a' + 26,
            false => c - b'A',
        };
        array[(index as usize)] = true;
    }

    for c in second.bytes() {
        let index = match c.is_ascii_lowercase() {
            true => c - b'a' + 26,
            false => c - b'A',
        };
        if array[(index as usize)] {
            return Some(c);
        }
    }
    return None;
}

fn part2(input: &str) {
    let mut sum = 0;
    for lines in input.lines().collect::<Vec<&str>>().chunks_mut(3) {
        let common = get_common_3(lines[2], lines[1], lines[0]).expect("should find common char");
        sum += match common.is_ascii_uppercase() {
            true => (common - b'A') as i32 + 1 + 26,
            false => (common - b'a') as i32 + 1,
        };
    }
    println!("{}", sum);
}

fn get_common_3(first: &str, second: &str, third: &str) -> Option<u8> {
    let mut array: [i32; 52] = [0; 52];
    for c in first.bytes() {
        let index = match c.is_ascii_lowercase() {
            true => c - b'a' + 26,
            false => c - b'A',
        };
        if array[(index as usize)] == 0 {
            array[(index as usize)] += 1;
        }
    }

    for c in second.bytes() {
        let index = match c.is_ascii_lowercase() {
            true => c - b'a' + 26,
            false => c - b'A',
        };
        if array[(index as usize)] == 1 {
            array[(index as usize)] += 1;
        }
    }

    for c in third.bytes() {
        let index = match c.is_ascii_lowercase() {
            true => c - b'a' + 26,
            false => c - b'A',
        };
        if array[(index as usize)] == 2 {
            return Some(c);
        }
    }
    return None;
}
