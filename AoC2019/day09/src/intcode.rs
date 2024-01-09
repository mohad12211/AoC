use std::collections::VecDeque;

pub struct Program {
    pub memory: Vec<i64>,
    ip: usize,
    relative_base: i64,
}

impl Program {
    pub fn new(memory: &[i64]) -> Self {
        Self {
            memory: memory.to_vec(),
            ip: 0,
            relative_base: 0,
        }
    }
    pub fn run(&mut self, input: Vec<i64>) -> (Vec<i64>, bool) {
        let mut input = VecDeque::from(input);
        let mut output = Vec::new();
        loop {
            let instruction = self.read(self.ip);
            let (opcode, parameters_modes) = self.parse_instruction(instruction);
            match opcode {
                99 => break,
                1 => {
                    let src1 = self.get_read_parameter(1, &parameters_modes);
                    let src2 = self.get_read_parameter(2, &parameters_modes);
                    let dst = self.get_write_parameter(3, &parameters_modes);
                    self.write(dst, src1 + src2);
                    self.ip += 4;
                }
                2 => {
                    let src1 = self.get_read_parameter(1, &parameters_modes);
                    let src2 = self.get_read_parameter(2, &parameters_modes);
                    let dst = self.get_write_parameter(3, &parameters_modes);
                    self.write(dst, src1 * src2);
                    self.ip += 4;
                }
                3 => {
                    let dst = self.get_write_parameter(1, &parameters_modes);
                    if let Some(input_value) = input.pop_front() {
                        self.write(dst, input_value);
                    } else {
                        return (output, false);
                    }
                    self.ip += 2;
                }
                4 => {
                    let output_value = self.get_read_parameter(1, &parameters_modes);
                    self.ip += 2;
                    output.push(output_value);
                }
                5 => {
                    let cond = self.get_read_parameter(1, &parameters_modes);
                    if cond != 0 {
                        let next_ip = self.get_read_parameter(2, &parameters_modes);
                        self.ip = next_ip as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let cond = self.get_read_parameter(1, &parameters_modes);
                    if cond == 0 {
                        let next_ip = self.get_read_parameter(2, &parameters_modes);
                        self.ip = next_ip as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let p1 = self.get_read_parameter(1, &parameters_modes);
                    let p2 = self.get_read_parameter(2, &parameters_modes);
                    let dst = self.get_write_parameter(3, &parameters_modes);
                    if p1 < p2 {
                        self.write(dst, 1);
                    } else {
                        self.write(dst, 0);
                    }
                    self.ip += 4;
                }
                8 => {
                    let p1 = self.get_read_parameter(1, &parameters_modes);
                    let p2 = self.get_read_parameter(2, &parameters_modes);
                    let dst = self.get_write_parameter(3, &parameters_modes);
                    if p1 == p2 {
                        self.write(dst, 1);
                    } else {
                        self.write(dst, 0);
                    }
                    self.ip += 4;
                }
                9 => {
                    let diff = self.get_read_parameter(1, &parameters_modes);
                    self.relative_base += diff;
                    self.ip += 2;
                }
                _ => unreachable!(),
            };
        }
        (output, true)
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
    fn get_read_parameter(&mut self, parameter_pos: usize, parameters_modes: &[i64]) -> i64 {
        let parameter_mode = *parameters_modes.get(parameter_pos - 1).unwrap_or(&0);
        let parameter = self.read(self.ip + parameter_pos);
        match parameter_mode {
            0 => self.read(parameter as usize),
            1 => parameter,
            2 => self.read((parameter + self.relative_base) as usize),
            _ => unreachable!(),
        }
    }

    fn get_write_parameter(&mut self, parameter_pos: usize, parameters_modes: &[i64]) -> usize {
        let parameter_mode = *parameters_modes.get(parameter_pos - 1).unwrap_or(&0);
        let parameter = self.read(self.ip + parameter_pos);
        match parameter_mode {
            0 => parameter as usize,
            1 => unreachable!("Value mode not allowed when writing"),
            2 => (parameter + self.relative_base) as usize,
            _ => unreachable!(),
        }
    }
    fn read(&mut self, addr: usize) -> i64 {
        self.verify_memory_capacity(addr);
        self.memory[addr]
    }
    fn write(&mut self, addr: usize, value: i64) {
        self.verify_memory_capacity(addr);
        self.memory[addr] = value;
    }

    fn verify_memory_capacity(&mut self, addr: usize) {
        if self.memory.len() <= addr {
            self.memory.extend(vec![0; addr - self.memory.len() + 1]);
        }
    }
}
