fn parse_instruction(ins: i64) -> (i64, Vec<i64>) {
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

fn extract(parameter: i64, mode: i64, memory: &[i64]) -> i64 {
    if mode == 0 {
        memory[parameter as usize] as i64
    } else if mode == 1 {
        parameter as i64
    } else {
        unreachable!()
    }
}

fn run_program(program: &[i64], input: i64) {
    let mut memory = program.to_vec();
    let mut ip = 0;

    while let Some(&instruction) = memory.get(ip) {
        let (opcode, parameters_mode) = parse_instruction(instruction);
        match opcode {
            99 => break,
            1 => {
                let src1 = extract(
                    memory[ip + 1],
                    *parameters_mode.get(0).unwrap_or(&0),
                    &memory,
                );
                let src2 = extract(
                    memory[ip + 2],
                    *parameters_mode.get(1).unwrap_or(&0),
                    &memory,
                );
                let dst = memory[ip + 3] as usize;
                memory[dst] = (src1 + src2) as i64;
                ip += 4;
            }
            2 => {
                let src1 = extract(
                    memory[ip + 1],
                    *parameters_mode.get(0).unwrap_or(&0),
                    &memory,
                );
                let src2 = extract(
                    memory[ip + 2],
                    *parameters_mode.get(1).unwrap_or(&0),
                    &memory,
                );
                let dst = memory[ip + 3] as usize;
                memory[dst] = (src1 * src2) as i64;
                ip += 4;
            }
            3 => {
                let dst = memory[ip + 1] as usize;
                memory[dst] = input;
                ip += 2;
            }
            4 => {
                let output = extract(
                    memory[ip + 1],
                    *parameters_mode.get(0).unwrap_or(&0),
                    &memory,
                );
                println!("{}", output);
                ip += 2;
            }
            _ => unreachable!(),
        };
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    run_program(&program, 1)
}
