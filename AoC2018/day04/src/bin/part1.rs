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
    let (most_mins_guard_id, most_mins_guard_actions) = guards_logs
        .into_iter()
        .map(|(id, actions)| {
            (
                id,
                actions
                    .chunks(2)
                    .map(|pair| pair[0]..pair[1])
                    .collect::<Vec<_>>(),
            )
        })
        .max_by_key(|(_, actions)| actions.iter().map(|r| r.len()).sum::<usize>())
        .unwrap();

    let mut mins_hist = [0; 60];
    for minute_range in most_mins_guard_actions {
        for n in minute_range {
            mins_hist[n] += 1;
        }
    }
    let (most_spent_min, _) = mins_hist
        .into_iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap();

    println!("{}", most_spent_min * most_mins_guard_id);
}
