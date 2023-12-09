use std::str::FromStr;

enum Direction {
    Left,
    Right,
}

enum Operation {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    Rotate(Direction, usize),
    RotateOnLetterReverse(char),
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
            Operation::RotateOnLetterReverse(char) => {
                // oldpos newpos   reverse with lookup table
                //    0      1  -> index 1 does 1 steps left
                //    1      3  -> index 3 does 2 steps left
                //    2      5  -> index 5 does 3 steps left
                //    3      7  -> index 7 does 4 steps left
                //    4      2  -> index 2 does 2 steps right = 6 left
                //    5      4  -> index 4 does 1 steps right = 7 left
                //    6      6  -> index 6 does 0 steps left
                //    7      0  -> index 0 does 7 steps right = 1 left
                let map = [1, 1, 6, 2, 7, 3, 0, 4];
                let index = s.chars().position(|c| c == char).unwrap();
                let rotations_to_reverse = map[index];
                Operation::Rotate(Direction::Left, rotations_to_reverse).apply(s.clone())
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
            // reverse direction
            ("rotate", "left") => Operation::Rotate(Direction::Right, parts[2].parse().unwrap()),
            // reverse direction
            ("rotate", "right") => Operation::Rotate(Direction::Left, parts[2].parse().unwrap()),
            // use reversed variant
            ("rotate", "based") => Operation::RotateOnLetterReverse(parts[6].parse().unwrap()),
            ("reverse", "positions") => {
                Operation::Reverse(parts[2].parse().unwrap(), parts[4].parse().unwrap())
            }
            // reverse indicies
            ("move", "position") => {
                Operation::Move(parts[5].parse().unwrap(), parts[2].parse().unwrap())
            }
            _ => unreachable!(),
        };
        Ok(operation)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let scrambled = String::from("fbgdceah");
    let password = input
        .lines()
        .map(|l| Operation::from_str(l).unwrap())
        .rev()
        .fold(scrambled, |acc, op| op.apply(acc));
    println!("{password}");
}
