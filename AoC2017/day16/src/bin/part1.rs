fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut programs: Vec<u8> = (b'a'..=b'p').collect();

    for line in input.trim().split(',') {
        match &line[0..1] {
            "s" => {
                let n: usize = line[1..].parse().unwrap();
                let (left, right) = programs.split_at(programs.len() - n);
                programs = right.iter().chain(left).copied().collect();
            }
            "x" => {
                let (a, b): (usize, usize) = line[1..]
                    .split_once('/')
                    .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                    .unwrap();
                programs.swap(a, b);
            }
            "p" => {
                let (a, b): (u8, u8) = line[1..]
                    .split_once('/')
                    .map(|(a, b)| (a.as_bytes()[0], b.as_bytes()[0]))
                    .unwrap();
                for p in programs.iter_mut() {
                    if *p == a {
                        *p = b;
                    } else if *p == b {
                        *p = a;
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    println!("{}", String::from_utf8_lossy(&programs));
}
