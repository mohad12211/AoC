#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_len: i64,
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for lens in input.trim().split(',') {
        let (label, focal_len) = lens.split_once(['=', '-']).unwrap();
        let box_index = hash(label);
        if !focal_len.is_empty() {
            if let Some(lens_in_box) = boxes[box_index].iter_mut().find(|lens| lens.label == label)
            {
                lens_in_box.focal_len = focal_len.parse().unwrap();
            } else {
                boxes[box_index].push(Lens {
                    label: label.to_string(),
                    focal_len: focal_len.parse().unwrap(),
                });
            }
        } else if let Some(i) = boxes[box_index].iter().position(|lens| lens.label == label) {
            boxes[box_index].remove(i);
        }
    }

    let sum: i64 = boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| {
                    ((box_index + 1) * (lens_index + 1)) as i64 * lens.focal_len
                })
                .sum::<i64>()
        })
        .sum();
    println!("{sum}");
}

fn hash(s: &str) -> usize {
    let mut current: usize = 0;
    for &b in s.as_bytes() {
        current += b as usize;
        current *= 17;
        current %= 256;
    }
    current
}
