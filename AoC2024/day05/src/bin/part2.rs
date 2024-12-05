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

    let mut result = 0;
    for line in updates.lines() {
        let mut numbers: Vec<_> = line.split(',').collect();
        if numbers.iter().enumerate().any(|(index, number)| {
            rules
                .get(number)
                .cloned()
                .unwrap_or_default()
                .iter()
                .any(|n| numbers[..index].contains(n))
        }) {
            let mut changed = true;
            while changed {
                changed = false;
                for index in 0..numbers.len() {
                    let number = numbers[index];
                    if let Some(target) = rules
                        .get(number)
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                        .flat_map(|n| numbers[..index].iter().position(|un| un == n))
                        .next()
                    {
                        numbers.swap(target, index);
                        changed = true;
                    }
                }
            }
            result += numbers[numbers.len() / 2].parse::<i32>().unwrap();
        }
    }

    println!("{result}");
}
