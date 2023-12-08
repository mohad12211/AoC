fn main() {
    let initial = "11011110011011101";
    let len = 272;

    let mut x: Vec<u8> = initial
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    while x.len() < len {
        x = dcurve(x);
    }
    if x.len() > len {
        x.truncate(len);
    }

    while x.len() % 2 == 0 {
        x = checksum(x);
    }

    println!("{}", x.iter().map(|n| n.to_string()).collect::<String>());
}

fn dcurve(v: Vec<u8>) -> Vec<u8> {
    let mut a = v;
    let mut b = a.clone();
    b.reverse();
    b.iter_mut().for_each(|n| *n = *n ^ 1);
    a.push(0);
    a.append(&mut b);
    a
}

fn checksum(v: Vec<u8>) -> Vec<u8> {
    v.chunks(2)
        .map(|c| if c[0] == c[1] { 1 } else { 0 })
        .collect()
}
