fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let steps: usize = input.trim().parse().unwrap();
    let mut i = 0;
    let mut len = 1;
    let mut after_zero = 0;

    for n in 1..=50000000 {
        i = (i + steps) % len;
        if i == 0 {
            after_zero = n;
        }
        len += 1;
        i += 1;
    }

    println!("{}", after_zero);
}
