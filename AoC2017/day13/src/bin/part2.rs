use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut layers: HashMap<usize, usize> = HashMap::new();

    let mut max = 0;
    for line in input.lines() {
        let (layer, range) = line.split_once(": ").unwrap();
        let layer = layer.parse().unwrap();
        layers.insert(layer, range.parse().unwrap());
        max = layer;
    }

    let mut wait_time = 0;
    'outer: loop {
        for i in (0 + wait_time)..=(max + wait_time) {
            let Some(&range) = layers.get(&(i - wait_time)) else {
                continue;
            };
            let mut pos = i % ((range - 1) * 2);
            if pos >= range {
                pos = range - (pos - range + 2);
            }

            if pos == 0 {
                wait_time += 1;
                continue 'outer;
            }
        }
        break;
    }
    println!("{wait_time}");
}
