fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let steps: usize = input.trim().parse().unwrap();
    let mut i = 0;
    let mut buf = vec![0];

    for n in 1..=2017 {
        i = (i + steps) % buf.len();
        buf.insert(i + 1, n);
        i += 1;
    }
    i = (i + 1) % buf.len();

    println!("{}", buf[i]);
}
