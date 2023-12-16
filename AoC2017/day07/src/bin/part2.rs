use std::{collections::HashMap, process::exit};

struct Program<'a> {
    weight: i32,
    children: Vec<&'a str>,
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");

    let mut map = HashMap::new();
    let root_program_name = get_root(input);

    for line in input.lines() {
        let mut parts = line.splitn(3, ' ');
        let node = parts.next().unwrap();
        let weight = parts
            .next()
            .and_then(|s| s.strip_prefix('('))
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap();
        let children = parts
            .next()
            .and_then(|s| s.strip_prefix("-> "))
            .map(|s| s.split(", ").collect::<Vec<_>>())
            .unwrap_or_default();
        map.insert(node, Program { weight, children });
    }
    calculate_weight_exit_if_find_unbalanced(root_program_name, &map);
}

fn calculate_weight_exit_if_find_unbalanced(
    program_name: &str,
    map: &HashMap<&str, Program>,
) -> i32 {
    let program = &map[program_name];
    let mut children_weights = Vec::new();
    for child in &program.children {
        children_weights.push(calculate_weight_exit_if_find_unbalanced(child, map));
    }
    if let Some((index, different)) = children_weights.iter().enumerate().find(|(_, &w)| {
        // check if there's an element in the vec that's different from the others
        // https://codereview.stackexchange.com/a/223838
        if children_weights[0] != children_weights[1] {
            w != children_weights[2]
        } else {
            w != children_weights[0]
        }
    }) {
        let difference = children_weights.iter().find(|&w| w != different).unwrap() - different;
        let balanced_weight = map[program.children[index]].weight + difference;
        println!("{balanced_weight}");
        exit(0);
    }
    children_weights.iter().sum::<i32>() + program.weight
}

fn get_root(input: &str) -> &str {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.splitn(4, ' ');
        let node = parts.next().unwrap();
        for child in parts.nth(2).unwrap_or_default().split(", ") {
            map.entry(child)
                .and_modify(|is_root| *is_root = false)
                .or_insert(false);
        }
        map.entry(node).or_insert(true);
    }

    map.into_iter().find(|&(_, is_root)| is_root).unwrap().0
}
