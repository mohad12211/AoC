fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut jumps: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut ip = 0;
    let mut counter = 0;

    while let Some(&offset) = jumps.get(ip) {
        counter += 1;
        let Some(new_ip) = ip.checked_add_signed(offset) else {
            break;
        };
        jumps[ip] += 1;
        ip = new_ip;
    }

    println!("{counter}");
}
