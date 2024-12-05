use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let (rules_input, updates) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in rules_input.lines() {
        let (left, right) = line.split_once("|").unwrap();
        rules
            .entry(left)
            .and_modify(|x| x.push(right))
            .or_insert(vec![right]);
    }

    let mut allowed = Vec::new();
    for line in updates.lines() {
        let numbers: Vec<_> = line.split(',').collect();
        if numbers.iter().enumerate().all(|(index, number)| {
            rules
                .get(number)
                .cloned()
                .unwrap_or_default()
                .iter()
                .all(|n| !numbers[..=index].contains(n))
        }) {
            allowed.push(numbers);
        }
    }

    let result: i32 = allowed
        .into_iter()
        .map(|l| l[l.len() / 2].parse::<i32>().unwrap())
        .sum();

    println!("{result}");
}
