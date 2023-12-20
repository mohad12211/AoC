use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, Copy)]
enum ModuleKind {
    FlipFlop,
    Conjuction,
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    kind: ModuleKind,
    inputs: HashMap<String, usize>,
    outputs: Vec<String>,
    state: usize,
}
impl Module {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut response_pulses = Vec::new();
        match self.kind {
            ModuleKind::FlipFlop => {
                if pulse.pulse == 0 {
                    self.state ^= 1;
                    for output in &self.outputs {
                        response_pulses.push(Pulse {
                            src: self.name.clone(),
                            dst: output.to_string(),
                            pulse: self.state,
                        });
                    }
                }
            }
            ModuleKind::Conjuction => {
                self.state = pulse.pulse;
                let x = self.inputs.get_mut(&pulse.src).unwrap();
                *x = pulse.pulse;
                if self.inputs.values().all(|&i| i == 1) {
                    for output in &self.outputs {
                        response_pulses.push(Pulse {
                            src: self.name.clone(),
                            dst: output.to_string(),
                            pulse: 0,
                        });
                    }
                } else {
                    for output in &self.outputs {
                        response_pulses.push(Pulse {
                            src: self.name.clone(),
                            dst: output.to_string(),
                            pulse: 1,
                        });
                    }
                }
            }
            ModuleKind::Broadcast => {
                for output in &self.outputs {
                    response_pulses.push(Pulse {
                        src: self.name.clone(),
                        dst: output.to_string(),
                        pulse: pulse.pulse,
                    });
                }
            }
        }
        response_pulses
    }
}

#[derive(Debug, Clone)]
struct Pulse {
    src: String,
    pulse: usize,
    dst: String,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.src,
            if self.pulse == 0 { "low" } else { "high" },
            self.dst
        )
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut modules = HashMap::new();
    for line in input.lines() {
        let (mut left, right) = line.split_once(" -> ").unwrap();
        let kind = match &left[0..1] {
            "%" => {
                left = left.strip_prefix('%').unwrap();
                ModuleKind::FlipFlop
            }
            "&" => {
                left = left.strip_prefix('&').unwrap();
                ModuleKind::Conjuction
            }
            "b" => ModuleKind::Broadcast,
            _ => unreachable!(),
        };
        let m = Module {
            name: left.to_string(),
            kind,
            inputs: HashMap::new(),
            outputs: right.split(", ").map(str::to_string).collect(),
            state: 0,
        };
        modules.insert(left.to_string(), m);
    }

    for line in input.lines() {
        let (mut left, right) = line.split_once(" -> ").unwrap();
        if let Some(stripped_left) = left.strip_prefix(['&', '%']) {
            left = stripped_left;
        }
        for module in right.split(", ") {
            if let Some(m) = modules.get_mut(module) {
                m.inputs.insert(left.to_string(), 0);
            }
        }
    }

    let mut pulses = VecDeque::new();
    for i in 0.. {
        let mut low_rx_count = 0;
        let mut high_rx_count = 0;

        pulses.push_back(Pulse {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            pulse: 0,
        });

        while let Some(pulse) = pulses.pop_front() {
            let Some(dst_module) = modules.get_mut(&pulse.dst) else {
                if pulse.pulse == 0 {
                    low_rx_count += 1;
                } else {
                    high_rx_count += 1;
                }
                continue;
            };
            let response_pulses = dst_module.process_pulse(pulse);
            response_pulses
                .into_iter()
                .for_each(|p| pulses.push_back(p));
        }

        if low_rx_count == 1 {
            println!("{i}");
            break;
        } else {
            println!("{low_rx_count}/{high_rx_count}");
        }
    }
}
