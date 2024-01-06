fn run_program(program: &[i64], input: [i64; 2]) -> i64 {
    let mut memory = program.to_vec();
    let mut ip = 0;

    memory[1..=2].copy_from_slice(&input);

    while let Some(&instruction) = memory.get(ip) {
        match instruction {
            99 => break,
            1 => {
                let src1 = memory[ip + 1] as usize;
                let src2 = memory[ip + 2] as usize;
                let dst = memory[ip + 3] as usize;
                memory[dst] = (memory[src1] + memory[src2]) as i64;
            }
            2 => {
                let src1 = memory[ip + 1] as usize;
                let src2 = memory[ip + 2] as usize;
                let dst = memory[ip + 3] as usize;
                memory[dst] = (memory[src1] * memory[src2]) as i64;
            }
            _ => unreachable!(),
        };
        ip += 4;
    }

    return memory[0];
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    println!("{}", run_program(&program, [12, 2]));
}
