fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut part = input;
    let mut result: u64 = 0;
    let mut enabled = true;
    loop {
        if part.starts_with("mul(") {
            part = &part[4..];
            let mut num1 = String::new();
            let mut index = 0;
            while part.as_bytes()[index].is_ascii_digit() {
                num1.push(part.chars().nth(index).unwrap());
                index += 1;
            }
            part = &part[num1.len()..];
            if !part.starts_with(',') || num1.is_empty() {
                continue;
            }
            part = &part[1..];

            let mut num2 = String::new();
            let mut index = 0;
            while part.as_bytes()[index].is_ascii_digit() {
                num2.push(part.chars().nth(index).unwrap());
                index += 1;
            }
            part = &part[num2.len()..];
            if !part.starts_with(')') || num2.is_empty() {
                continue;
            }
            part = &part[1..];

            if enabled {
                result += num1.parse::<u64>().unwrap() * num2.parse::<u64>().unwrap();
            }
        } else if part.starts_with("do()") {
            enabled = true;
            part = &part[4..];
        } else if part.starts_with("don't()") {
            enabled = false;
            part = &part[7..];
        } else if let Some(rest) = part.get(1..) {
            part = rest;
        } else {
            break;
        }
    }
    println!("{result}");
}
