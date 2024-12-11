fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut is_file = true;
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut spaces: Vec<(usize, usize)> = Vec::new();
    let mut files: Vec<(usize, usize, usize)> = Vec::new();
    for (i, char) in input.trim().chars().enumerate() {
        let id = i / 2;
        let num = char.to_digit(10).unwrap();
        for _ in 0..num {
            if is_file {
                if disk
                    .last()
                    .is_some_and(|part| part.is_some_and(|file_id| file_id == id))
                {
                    files.last_mut().unwrap().2 += 1;
                } else {
                    files.push((disk.len(), id, 1));
                }
                disk.push(Some(id));
            } else {
                if disk.last().is_some_and(|part| part.is_none()) {
                    spaces.last_mut().unwrap().1 += 1;
                } else {
                    spaces.push((disk.len(), 1));
                }
                disk.push(None);
            }
        }
        is_file = !is_file;
    }

    loop {
        let Some((file_index, _, file_len)) = files.pop() else {
            break;
        };
        if let Some((space_index, space_len)) = spaces
            .iter_mut()
            .find(|(index, len)| *len >= file_len && *index < file_index)
        {
            for i in 0..file_len {
                disk.swap(file_index + i, *space_index + i);
            }
            *space_len -= file_len;
            *space_index += file_len;
        }
    }

    let mut result = 0;

    for (i, file) in disk.iter().enumerate() {
        let &Some(id) = file else {
            continue;
        };
        result += id * i;
    }

    println!("{result}");
}
