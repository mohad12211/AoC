fn main() {
    let input = include_str!("input.txt");

    let mut map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut count = 0;

    loop {
        let mut rolls = Vec::new();
        for r in 0..map.len() {
            for c in 0..map[r].len() {
                if map[r][c] != b'@' {
                    continue;
                }

                let mut adj_count = 0;
                let dirs = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                for (dr, dc) in dirs {
                    let nr = r.wrapping_add_signed(dr);
                    let nc = c.wrapping_add_signed(dc);

                    if let Some(&b'@') = map.get(nr).and_then(|row| row.get(nc)) {
                        adj_count += 1;
                    }
                }

                if adj_count < 4 {
                    rolls.push((r, c));
                }
            }
        }

        if rolls.is_empty() {
            break;
        }

        count += rolls.len();

        for (r, c) in rolls {
            map[r][c] = b'.';
        }
    }

    println!("{count}");
}
