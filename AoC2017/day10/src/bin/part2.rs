fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let size = 256;
    let mut nums: Vec<_> = (0..size).collect();

    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in input
            .trim()
            .as_bytes()
            .iter()
            .copied()
            .chain([17, 31, 73, 47, 23])
        {
            let length = length as usize;
            let start = current_position;
            let end = current_position + length;

            if end > size {
                let higher_slice = &nums[start..];
                let lower_slice = &nums[..(end % size)];
                let mut full_slice = [higher_slice, lower_slice].concat();
                full_slice.reverse();
                nums[start..].copy_from_slice(&full_slice[..(size - start)]);
                nums[..(end % size)].copy_from_slice(&full_slice[(size - start)..]);
            } else {
                nums[start..end].reverse();
            }

            current_position = (current_position + length + skip_size) % size;
            skip_size += 1;
        }
    }

    let hex: String = nums
        .chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|acc, n| acc ^ n).unwrap())
        .map(|x| format!("{:02x}", x))
        .collect();
    println!("{hex}");
}
