use md5::compute;

fn main() {
    let salt = "cuanljph";
    let mut index: i64 = 0;
    let mut key_count = 0;
    while key_count != 64 {
        let hash = format!("{:?}", compute(format!("{salt}{index}")));
        if let Some(c) = has_n(&hash, 3, None) {
            for i in (index + 1)..=(index + 1000) {
                let hash = format!("{:?}", compute(format!("{salt}{i}")));
                if has_n(&hash, 5, Some(c)).is_some() {
                    key_count += 1;
                    break;
                }
            }
        }

        index += 1;
    }

    println!("{}", index - 1);
}

fn has_n(s: &str, n: usize, c: Option<char>) -> Option<char> {
    for i in 0..(s.len() - n + 1) {
        let c = c.unwrap_or(s.chars().nth(i).unwrap());
        if s.chars().skip(i).take(n).all(|c1| c1 == c) {
            return Some(c);
        }
    }
    None
}
