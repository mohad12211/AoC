fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let stream = input.trim().as_bytes();
    let mut nesting_counter = 0;
    let mut i = 0;
    let mut total_score = 0;

    while i < stream.len() {
        let c = stream[i];
        match c {
            b'{' => nesting_counter += 1,
            b'}' => {
                total_score += nesting_counter;
                nesting_counter -= 1;
            }
            b'<' => {
                i += 1;
                while stream[i] != b'>' {
                    match stream[i] {
                        b'!' => i += 1,
                        b'>' => break,
                        _ => {}
                    }
                    i += 1;
                }
            }
            b'!' => i += 1,
            b',' => {}
            _ => unreachable!(),
        };
        i += 1;
    }

    println!("{total_score}");
}
