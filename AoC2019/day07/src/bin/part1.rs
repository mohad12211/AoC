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
    for perm in permutations(&mut vec![0, 1, 2, 3, 4]) {
        let mut prev_output = 0;
        for phase in perm {
            let mut program = Program::new(&memory);
            prev_output = program.run(vec![phase, prev_output]).unwrap();
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
