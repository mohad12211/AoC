use day07::intcode::Program;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let mut max = 0;
    for phases in permutations(&mut vec![5, 6, 7, 8, 9]) {
        let mut programs = Vec::new();
        let mut prev_output = 0;

        for phase in phases.into_iter() {
            let mut program = Program::new(&memory);
            prev_output = program.run(vec![phase, prev_output]).unwrap();
            programs.push(program);
        }

        'outer: loop {
            for program in programs.iter_mut() {
                let Some(output) = program.run(vec![prev_output]) else {
                    break 'outer;
                };
                prev_output = output;
            }
        }

        max = max.max(prev_output);
    }
    println!("{max}");
}

fn permutations<T: Clone>(input: &mut Vec<T>) -> Vec<Vec<T>> {
    fn generate_permutations<T: Clone>(input: &mut Vec<T>, start: usize, result: &mut Vec<Vec<T>>) {
        if start == input.len() {
            result.push(input.clone());
            return;
        }

        for i in start..input.len() {
            input.swap(start, i);
            generate_permutations(input, start + 1, result);
            input.swap(start, i);
        }
    }

    let mut result = Vec::new();
    generate_permutations(input, 0, &mut result);
    result
}
