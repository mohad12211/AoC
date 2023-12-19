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

    let mut severity = 0;
    for i in 0..=max {
        let Some(&range) = layers.get(&i) else {
            continue;
        };
        let mut pos = i % ((range - 1) * 2);
        if pos >= range {
            pos = range - (pos - range + 2);
        }

        if pos == 0 {
            severity += i * range;
        }
    }

    println!("{severity}");
}
