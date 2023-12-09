use std::str::FromStr;

enum Direction {
    Left,
    Right,
}

enum Operation {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    Rotate(Direction, usize),
    RotateOnLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn apply(self, s: String) -> String {
        match self {
            Operation::SwapPositions(i, j) => s
                .char_indices()
                .map(|(index, c)| {
                    if index == i {
                        s.chars().nth(j).unwrap()
                    } else if index == j {
                        s.chars().nth(i).unwrap()
                    } else {
                        c
                    }
                })
                .collect(),
            Operation::SwapLetters(x, y) => s
                .chars()
                .map(|c| {
                    if c == x {
                        y
                    } else if c == y {
                        x
                    } else {
                        c
                    }
                })
                .collect(),
            Operation::Rotate(dir, mut steps) => {
                steps %= s.len();
                match dir {
                    Direction::Left => {
                        let mut end = s[steps..].to_string();
                        end.push_str(&s[0..steps]);
                        end
                    }
                    Direction::Right => {
                        let mut start = s[(s.len() - steps)..].to_string();
                        start.push_str(&s[0..(s.len() - steps)]);
                        start
                    }
                }
            }
            Operation::RotateOnLetter(char) => {
                let mut index = s.chars().position(|c| c == char).unwrap();
                if index >= 4 {
                    index += 1;
                }
                Operation::Rotate(Direction::Right, 1 + index).apply(s)
            }
            Operation::Reverse(from, to) => {
                let (rest, end) = s.split_at(to + 1);
                let (start, middle) = rest.split_at(from);
                format!("{start}{}{end}", middle.chars().rev().collect::<String>())
            }
            Operation::Move(from, to) => {
                let mut result = s;
                let char = result.remove(from);
                result.insert(to, char);
                result
            }
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();

        let operation = match (parts[0], parts[1]) {
            ("swap", "position") => {
                Operation::SwapPositions(parts[2].parse().unwrap(), parts[5].parse().unwrap())
            }
            ("swap", "letter") => {
                Operation::SwapLetters(parts[2].parse().unwrap(), parts[5].parse().unwrap())
            }
            ("rotate", "left") => Operation::Rotate(Direction::Left, parts[2].parse().unwrap()),
            ("rotate", "right") => Operation::Rotate(Direction::Right, parts[2].parse().unwrap()),
            ("rotate", "based") => Operation::RotateOnLetter(parts[6].parse().unwrap()),
            ("reverse", "positions") => {
                Operation::Reverse(parts[2].parse().unwrap(), parts[4].parse().unwrap())
            }
            ("move", "position") => {
                Operation::Move(parts[2].parse().unwrap(), parts[5].parse().unwrap())
            }
            _ => unreachable!(),
        };
        Ok(operation)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let password = String::from("abcdefgh");
    let scrambled = input
        .lines()
        .map(|l| Operation::from_str(l).unwrap())
        .fold(password, |acc, op| op.apply(acc));
    println!("{scrambled}");
}
