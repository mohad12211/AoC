use std::collections::{HashMap, VecDeque};

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
                if pulse.pulse == 1 {
                    return response_pulses;
                }
                self.state ^= 1;
                for output in &self.outputs {
                    response_pulses.push(Pulse {
                        src: self.name.clone(),
                        dst: output.to_string(),
                        pulse: self.state,
                    });
                }
            }
            ModuleKind::Conjuction => {
                self.state = pulse.pulse;
                *self.inputs.get_mut(&pulse.src).unwrap() = pulse.pulse;
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

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut modules = HashMap::new();
    for line in input.lines() {
        let (mut module_name, module_outputs) = line.split_once(" -> ").unwrap();
        let kind = match &module_name[0..1] {
            "%" => {
                module_name = module_name.strip_prefix('%').unwrap();
                ModuleKind::FlipFlop
            }
            "&" => {
                module_name = module_name.strip_prefix('&').unwrap();
                ModuleKind::Conjuction
            }
            "b" => ModuleKind::Broadcast,
            _ => unreachable!(),
        };
        let module = Module {
            name: module_name.to_string(),
            kind,
            inputs: HashMap::new(),
            outputs: module_outputs.split(", ").map(String::from).collect(),
            state: 0,
        };
        modules.insert(module_name.to_string(), module);
    }

    for line in input.lines() {
        let (mut module_name, outputs) = line.split_once(" -> ").unwrap();
        if let Some(stripped_name) = module_name.strip_prefix(['&', '%']) {
            module_name = stripped_name;
        }
        for output in outputs.split(", ") {
            if let Some(module) = modules.get_mut(output) {
                module.inputs.insert(module_name.to_string(), 0);
            }
        }
    }

    let rx_conjunction = modules
        .iter()
        .find_map(|(name, module)| module.outputs.iter().any(|o| o == "rx").then_some(name))
        .unwrap();
    let rx_conjunction_inputs: Vec<_> = modules
        .iter()
        .filter_map(|(name, module)| {
            module
                .outputs
                .iter()
                .any(|o| o == rx_conjunction)
                .then_some(name.clone())
        })
        .collect();
    let mut conjunction_cycles = vec![0; rx_conjunction_inputs.len()];
    let mut pulse_queue = VecDeque::new();
    'outer: for i in 0.. {
        pulse_queue.push_back(Pulse {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            pulse: 0,
        });

        while let Some(pulse) = pulse_queue.pop_front() {
            if rx_conjunction_inputs.contains(&pulse.src) && pulse.pulse == 1 {
                let Some(input) = conjunction_cycles.iter_mut().find(|i| **i == 0) else {
                    break 'outer;
                };
                *input = i + 1;
            }
            let Some(dst_module) = modules.get_mut(&pulse.dst) else {
                continue;
            };
            for response_pulse in dst_module.process_pulse(pulse) {
                pulse_queue.push_back(response_pulse);
            }
        }
    }
    println!("{}", total_lcm(conjunction_cycles).unwrap());
}

fn total_lcm(nums: Vec<u64>) -> Option<u64> {
    nums.into_iter().reduce(lcm)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
