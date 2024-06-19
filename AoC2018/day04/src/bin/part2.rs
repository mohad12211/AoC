use std::collections::HashMap;

fn parse_time(s: &str) -> usize {
    let (_, after_colon) = s.split_once(':').unwrap();
    let (time, _) = after_colon.split_once(']').unwrap();
    time.parse().unwrap()
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut guards_logs: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut lines: Vec<_> = input.lines().collect();
    let bracket_index = lines[0].find(']').unwrap();
    lines.sort_by(|a, b| a[..bracket_index].cmp(&b[..bracket_index]));
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let (_, rest) = line.split_once('#').unwrap();
        let (id, _) = rest.split_once(' ').unwrap();
        let id = id.parse().unwrap();
        let mut actions = Vec::new();
        while lines.get(i + 1).is_some_and(|l| !l.contains('#')) {
            actions.push(parse_time(lines[i + 1]));
            i += 1;
        }
        guards_logs
            .entry(id)
            .and_modify(|saved_actions| saved_actions.extend(&actions))
            .or_insert(actions);
        i += 1;
    }
    let mut guards_hist: HashMap<usize, [i32; 60]> = HashMap::new();
    for (id, actions) in guards_logs {
        let mut hist = [0; 60];
        for pair in actions.chunks(2) {
            for n in pair[0]..pair[1] {
                hist[n] += 1;
            }
        }
        guards_hist.insert(id, hist);
    }
    let (most_freq_min_id, most_freq_min_hist) = guards_hist
        .iter()
        .max_by_key(|(_, hist)| hist.iter().max().unwrap())
        .unwrap();
    let (most_freq_min, _) = most_freq_min_hist
        .iter()
        .enumerate()
        .max_by_key(|(_, n)| **n)
        .unwrap();

    println!("{}", most_freq_min_id * most_freq_min);
}
