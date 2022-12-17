use std::vec;

fn main() {
    let input = include_str!("../inputs/day5.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    // parse
    let mut stacks: Vec<Vec<char>> = vec![];
    let input = input.split("\n\n").collect::<Vec<&str>>();
    (0..((input[0].chars().position(|c| c == '\n').unwrap()) + 1) / 4)
        .for_each(|_| stacks.push(vec![]));
    let mut s_index = 0;
    for e in input[0].chars().collect::<Vec<char>>().chunks(4) {
        if e[1].is_uppercase() {
            stacks[s_index].push(e[1]);
        }
        s_index += 1;
        if *e.last().unwrap() == '\n' {
            s_index = 0;
        }
    }
    stacks.iter_mut().for_each(|x| x.reverse());
    // move stuff
    input[1].lines().for_each(|l| {
        let ins = l.split(" ").collect::<Vec<&str>>();
        let times = ins[1].parse::<usize>().unwrap();
        let from = ins[3].parse::<usize>().unwrap() - 1;
        let to = ins[5].parse::<usize>().unwrap() - 1;
        (0..times).for_each(|_| {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
        });
    });
    println!(
        "{}",
        stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
    );
}

fn part2(input: &str) {
    // parse
    let mut stacks: Vec<Vec<char>> = vec![];
    let input = input.split("\n\n").collect::<Vec<&str>>();
    (0..((input[0].chars().position(|c| c == '\n').unwrap()) + 1) / 4)
        .for_each(|_| stacks.push(vec![]));
    let mut s_index = 0;
    for e in input[0].chars().collect::<Vec<char>>().chunks(4) {
        if e[1].is_uppercase() {
            stacks[s_index].push(e[1]);
        }
        s_index += 1;
        if *e.last().unwrap() == '\n' {
            s_index = 0;
        }
    }
    stacks.iter_mut().for_each(|x| x.reverse());
    // move stuff
    input[1].lines().for_each(|l| {
        let ins = l.split(" ").collect::<Vec<&str>>();
        let times = ins[1].parse::<usize>().unwrap();
        let from = ins[3].parse::<usize>().unwrap() - 1;
        let to = ins[5].parse::<usize>().unwrap() - 1;
        let mut items = (0..times)
            .map(|_| stacks[from].pop().unwrap())
            .collect::<Vec<char>>();
        items.reverse();
        stacks[to].append(&mut items);
    });
    println!(
        "{}",
        stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
    );
}
