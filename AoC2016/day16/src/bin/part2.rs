fn main() {
    let initial = "11011110011011101";
    let len = 35651584;
    let count = 2usize.pow(divide_by_two_count(len));

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

    let x = x
        .chunks(count)
        .map(|c| {
            if c.iter().filter(|c1| **c1 == 1).count() % 2 == 0 {
                1
            } else {
                0
            }
            .to_string()
        })
        .collect::<String>();

    println!("{}", x);
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

fn divide_by_two_count(mut n: usize) -> u32 {
    let mut counter = 0;
    while n % 2 == 0 {
        n = n / 2;
        counter += 1;
    }
    counter
}
