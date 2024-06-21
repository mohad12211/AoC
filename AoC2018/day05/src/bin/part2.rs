fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    const CAPITAL_MASK: u8 = 0b0010_0000;
    let polymer: Vec<_> = input.trim().bytes().collect();
    let mut min_len = usize::MAX;

    for removed_unit in b'a'..b'z' {
        let mut result = Vec::with_capacity(polymer.len());
        for u1 in polymer
            .iter()
            .filter(|&&u| (u | CAPITAL_MASK) != removed_unit)
        {
            if result.last().is_some_and(|u2| u1 ^ u2 == CAPITAL_MASK) {
                result.pop().unwrap();
            } else {
                result.push(*u1);
            }
        }
        min_len = min_len.min(result.len());
    }
    println!("{min_len}");
}
