struct Program {
    memory: Vec<i64>,
    ip: usize,
    input: Vec<i64>,
}

impl Program {
    fn run(&mut self) {
        while let Some(&instruction) = self.memory.get(self.ip) {
            let (opcode, parameters_modes) = self.parse_instruction(instruction);
            match opcode {
                99 => break,
                1 => {
                    let src1 = self.extract(1, &parameters_modes);
                    let src2 = self.extract(2, &parameters_modes);
                    let dst = self.memory[self.ip + 3] as usize;
                    self.memory[dst] = src1 + src2;
                    self.ip += 4;
                }
                2 => {
                    let src1 = self.extract(1, &parameters_modes);
                    let src2 = self.extract(2, &parameters_modes);
                    let dst = self.memory[self.ip + 3] as usize;
                    self.memory[dst] = src1 * src2;
                    self.ip += 4;
                }
                3 => {
                    let dst = self.memory[self.ip + 1] as usize;
                    self.memory[dst] = self.input[0];
                    self.ip += 2;
                }
                4 => {
                    let output = self.extract(1, &parameters_modes);
                    println!("{}", output);
                    self.ip += 2;
                }
                5 => {
                    let cond = self.extract(1, &parameters_modes);
                    if cond != 0 {
                        let dst = self.extract(2, &parameters_modes);
                        self.ip = dst as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let cond = self.extract(1, &parameters_modes);
                    if cond == 0 {
                        let dst = self.extract(2, &parameters_modes);
                        self.ip = dst as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let p1 = self.extract(1, &parameters_modes);
                    let p2 = self.extract(2, &parameters_modes);
                    let dst = self.memory[self.ip + 3];
                    if p1 < p2 {
                        self.memory[dst as usize] = 1;
                    } else {
                        self.memory[dst as usize] = 0;
                    }
                    self.ip += 4;
                }
                8 => {
                    let p1 = self.extract(1, &parameters_modes);
                    let p2 = self.extract(2, &parameters_modes);
                    let dst = self.memory[self.ip + 3];
                    if p1 == p2 {
                        self.memory[dst as usize] = 1;
                    } else {
                        self.memory[dst as usize] = 0;
                    }
                    self.ip += 4;
                }
                _ => unreachable!(),
            };
        }
    }
    fn parse_instruction(&self, ins: i64) -> (i64, Vec<i64>) {
        (
            ins % 100,
            (ins / 100)
                .to_string()
                .chars()
                .rev()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect(),
        )
    }
    fn extract(&self, parameter_pos: usize, parameters_modes: &[i64]) -> i64 {
        let parameter_mode = *parameters_modes.get(parameter_pos - 1).unwrap_or(&0);
        let parameter = self.memory[self.ip + parameter_pos];
        match parameter_mode {
            0 => self.memory[parameter as usize],
            1 => parameter,
            _ => unreachable!(),
        }
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let mut program = Program {
        memory,
        ip: 0,
        input: vec![5],
    };

    program.run();
}
