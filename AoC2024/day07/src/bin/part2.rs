enum Op {
    Plus,
    Mul,
    Concat,
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut result = 0;
    for line in input.lines() {
        let (total, number) = line.split_once(": ").unwrap();
        let total = total.parse().unwrap();
        let numbers: Vec<u64> = number.split(' ').map(|n| n.parse().unwrap()).collect();
        if evalutes(total, numbers[0], 1, &numbers, Op::Plus)
            || evalutes(total, numbers[0], 1, &numbers, Op::Mul)
            || evalutes(total, numbers[0], 1, &numbers, Op::Concat)
        {
            result += total;
        }
    }
    println!("{result}");
}

fn evalutes(total: u64, partial: u64, next_index: usize, numbers: &[u64], operator: Op) -> bool {
    if let Some(next) = numbers.get(next_index) {
        let next_partial = match operator {
            Op::Plus => partial + next,
            Op::Mul => partial * next,
            Op::Concat => format!("{partial}{next}").parse().unwrap(),
        };
        evalutes(total, next_partial, next_index + 1, numbers, Op::Mul)
            || evalutes(total, next_partial, next_index + 1, numbers, Op::Plus)
            || evalutes(total, next_partial, next_index + 1, numbers, Op::Concat)
    } else {
        partial == total
    }
}
