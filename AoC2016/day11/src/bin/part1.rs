use std::collections::VecDeque;
use Machine::*;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
enum Machine {
    Generator(u8),
    MicroChip(u8),
}

#[derive(Clone, Eq)]
struct Floor {
    machines: Vec<Machine>,
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        self.machines.iter().fold((0, 0), |(g, m), b| match b {
            Generator(_) => (g + 1, m),
            MicroChip(_) => (g, m + 1),
        }) == other.machines.iter().fold((0, 0), |(g, m), b| match b {
            Generator(_) => (g + 1, m),
            MicroChip(_) => (g, m + 1),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Move {
    df: i32,
    machines: Vec<Machine>,
}

#[derive(Clone, Eq, PartialEq)]
struct Lobby {
    floors: [Floor; 4],
    elevator: i32,
}

impl Lobby {
    fn is_valid_move(&self, m: &Move) -> bool {
        if m.df + self.elevator < 0 || m.df + self.elevator > 3 {
            return false;
        }
        let all_machines: Vec<&Machine> = m
            .machines
            .iter()
            .chain(&self.floors[(m.df + self.elevator) as usize].machines)
            .collect();
        let fry = Self::will_fry(&all_machines[..]);

        if fry {
            return false;
        }

        let all_machines: &Vec<Machine> = &self.floors[self.elevator as usize].machines;
        let fry = Self::will_fry(&all_machines.iter().collect::<Vec<_>>());

        if fry {
            return false;
        }

        true
    }

    fn apply_move(&mut self, mo: Move) {
        self.floors[self.elevator as usize]
            .machines
            .retain(|m| !mo.machines.contains(m));
        self.floors[(mo.df + self.elevator) as usize]
            .machines
            .extend(mo.machines);
        self.elevator += mo.df;
    }

    fn will_fry(machines: &[&Machine]) -> bool {
        let generators: Vec<_> = machines
            .iter()
            .filter_map(|m| match m {
                Generator(e) => Some(e),
                MicroChip(_) => None,
            })
            .collect();
        let mut microchips = machines.iter().filter_map(|m| match m {
            Generator(_) => None,
            MicroChip(e) => Some(e),
        });

        let unused_microchips = microchips.any(|m| !generators.contains(&m));

        unused_microchips && !generators.is_empty()
    }

    fn available_moves(&self) -> Vec<Move> {
        let current_machines = &self.floors[self.elevator as usize].machines;
        let mut moves = Vec::new();

        for i in 0..current_machines.len() {
            for j in i + 1..current_machines.len() {
                moves.push(Move {
                    df: 1,
                    machines: vec![current_machines[j], current_machines[i]],
                });
            }
        }

        for m in current_machines {
            moves.push(Move {
                df: 1,
                machines: vec![*m],
            });
        }

        for m in current_machines {
            moves.push(Move {
                df: -1,
                machines: vec![*m],
            });
        }

        moves
            .into_iter()
            .filter(|m| self.is_valid_move(m))
            .collect()
    }

    fn is_done(&self) -> bool {
        self.floors[3].machines.len() == 10
    }
}

fn bfs(lobby: Lobby) -> Option<i32> {
    let mut q = VecDeque::new();
    let mut seen = Vec::new();

    q.push_back((lobby, 0));

    while let Some((lobby, steps)) = q.pop_front() {
        if seen.contains(&lobby) {
            continue;
        }
        for m in lobby.available_moves() {
            let mut lobby = lobby.clone();
            lobby.apply_move(m);
            if lobby.is_done() {
                return Some(steps + 1);
            } else {
                q.push_back((lobby, steps + 1));
            }
        }
        seen.push(lobby);
    }

    None
}

fn main() {
    let floors = [
        Floor {
            machines: vec![
                Generator(b'S'),
                MicroChip(b'S'),
                Generator(b'P'),
                MicroChip(b'P'),
            ],
        },
        Floor {
            machines: vec![
                Generator(b'T'),
                Generator(b'R'),
                MicroChip(b'R'),
                Generator(b'C'),
                MicroChip(b'C'),
            ],
        },
        Floor {
            machines: vec![MicroChip(b'T')],
        },
        Floor { machines: vec![] },
    ];
    let lobby = Lobby {
        elevator: 0,
        floors,
    };

    let steps = bfs(lobby.clone()).unwrap();
    println!("{}", steps);
}
