const TABLE: [u8; 8] = [0, 1, 0, 1, 1, 0, 1, 0];

fn main() {
    let input = ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^";
    let row_count = 400000;

    let mut first_row = vec![0];
    first_row.extend(
        input
            .as_bytes()
            .iter()
            .map(|b| if *b == b'.' { 0 } else { 1 }),
    );
    first_row.push(0);

    let mut safe_tiles_count = 0;
    let mut row_index = 0;
    let mut current_row = first_row;
    while row_index < row_count {
        safe_tiles_count += current_row.iter().filter(|tile| **tile == 0).count() - 2;
        current_row = get_next_row(&current_row);
        row_index += 1;
    }
    println!("{safe_tiles_count}");
}

fn get_next_row(row: &Vec<u8>) -> Vec<u8> {
    let mut next_row = Vec::with_capacity(row.len());
    next_row.push(0);
    for i in 1..(row.len() - 1) {
        next_row.push(TABLE[to_num(&row[(i - 1)..=(i + 1)])]);
    }
    next_row.push(0);
    next_row
}

fn to_num(v: &[u8]) -> usize {
    v.iter()
        .copied()
        .reduce(|acc, n| (acc << 1) | n)
        .unwrap()
        .into()
}
