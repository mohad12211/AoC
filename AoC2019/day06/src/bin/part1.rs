use std::collections::HashMap;

fn orbit_count(object: &str, map: &HashMap<&str, &str>) -> usize {
    if object == "COM" {
        return 1;
    }
    return 1 + orbit_count(map[object], map);
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut map = HashMap::new();
    for line in input.lines() {
        let (center, object) = line.split_once(')').unwrap();
        map.insert(object, center);
    }

    let sum: usize = map.values().map(|&v| orbit_count(v, &map)).sum();
    println!("{sum}");
}
