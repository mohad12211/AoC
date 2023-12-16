fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let stream = input.trim().as_bytes();
    let mut i = 0;
    let mut garbage_count = 0;

    while i < stream.len() {
        let c = stream[i];
        match c {
            b'<' => {
                i += 1;
                while stream[i] != b'>' {
                    match stream[i] {
                        b'!' => i += 1,
                        b'>' => break,
                        _ => garbage_count += 1,
                    }
                    i += 1;
                }
            }
            b'!' => i += 1,
            b',' | b'{' | b'}' => {}
            _ => unreachable!(),
        };
        i += 1;
    }

    println!("{garbage_count}");
}
