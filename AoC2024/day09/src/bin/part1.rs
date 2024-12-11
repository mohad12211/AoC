fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut is_file = true;
    let mut disk = Vec::new();
    for (i, char) in input.trim().chars().enumerate() {
        let num = char.to_digit(10).unwrap();
        for _ in 0..num {
            if is_file {
                disk.push(Some(i / 2));
            } else {
                disk.push(None);
            }
        }
        is_file = !is_file;
    }

    let mut space = disk.iter().position(Option::is_none).unwrap();
    let mut last = disk.len() - 1;

    while space <= last {
        disk.swap(space, last);
        last -= 1;
        space += disk[space..].iter().position(Option::is_none).unwrap();
    }

    let mut result: usize = 0;

    for (i, file) in disk.iter().enumerate() {
        let &Some(id) = file else {
            break;
        };
        result += id * i;
    }

    println!("{result}");
}
