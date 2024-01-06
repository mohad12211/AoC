use day09::intcode::Program;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let mut program = Program::new(&memory);
    while let Some(output) = program.run(vec![1]) {
        println!("{output}");
    }
}
