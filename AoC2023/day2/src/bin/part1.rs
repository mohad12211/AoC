use std::str::FromStr;

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
    id: i32,
}

#[derive(Copy, Clone, Default, Debug)]
struct Round {
    r: i32,
    g: i32,
    b: i32,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s.split(", ");
        let mut round: Round = Default::default();
        for cube in cubes {
            let (cube_count, cube_color) = cube
                .split_once(' ')
                .ok_or("No space between cube count and color")?;
            let cube_count = cube_count
                .parse::<i32>()
                .map_err(|_| "Invalid cube count")?;
            match cube_color {
                "red" => round.r = cube_count,
                "green" => round.g = cube_count,
                "blue" => round.b = cube_count,
                _ => return Err("Invalid Color".into()),
            };
        }
        Ok(round)
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(": "))
            .map(|(id, rounds)| {
                (
                    id,
                    rounds.split("; ").map(Round::from_str).collect::<Vec<_>>(),
                )
            })
            .ok_or("Invalid Game")?;

        Ok(Self {
            id: id.parse().map_err(|_| "Invalid GameID")?,
            rounds: rounds.into_iter().collect::<Result<_, _>>()?,
        })
    }
}

fn main() {
    // let input = include_str!("./input.test.txt");
    let input = include_str!("./input.txt");
    let id_sum: i32 = input
        .lines()
        .map(|l| Game::from_str(l).unwrap())
        .filter(|g| g.rounds.iter().all(|x| x.r <= 12 && x.g <= 13 && x.b <= 14))
        .map(|x| x.id)
        .sum();
    println!("{id_sum}")
}
