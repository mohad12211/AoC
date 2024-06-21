fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let polymer: Vec<_> = input.trim().bytes().collect();
    let mut result = Vec::with_capacity(polymer.len());
    for u1 in &polymer {
        if result
            .last()
            .is_some_and(|u2| u1 ^ u2 == 0b0010_0000 /* capital bit */)
        {
            result.pop().unwrap();
        } else {
            result.push(*u1);
        }
    }
    println!("{}", result.len());
}
