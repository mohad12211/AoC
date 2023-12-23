use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let start = 1;
    let end = (width - 2) + height * (height - 1);

    let mut nodes = HashSet::new();
    nodes.insert(start);
    nodes.insert(end);

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                continue;
            }
            let mut neighbours = 0;
            let (x, y) = (x as i32, y as i32);
            for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if (0..width).contains(&nx)
                    && (0..height).contains(&ny)
                    && grid[ny as usize][nx as usize] != '#'
                {
                    neighbours += 1;
                }
            }
            if neighbours >= 3 {
                nodes.insert(x + y * height);
            }
        }
    }

    let mut graph = HashMap::new();
    for &node in &nodes {
        fill_graph(node, &nodes, &grid, &mut graph);
    }
    let mut seen = HashSet::new();
    let mut max_steps = 0;
    find_longest(start, end, &graph, &mut seen, 0, &mut max_steps);
    println!("{max_steps}");
}

fn find_longest(
    node: i32,
    end: i32,
    graph: &HashMap<i32, Vec<(i32, usize)>>,
    seen: &mut HashSet<i32>,
    steps: usize,
    max_steps: &mut usize,
) {
    if node == end {
        *max_steps = (*max_steps).max(steps);
        return;
    }

    seen.insert(node);
    for &(next_node, weight) in &graph[&node] {
        if !seen.contains(&next_node) {
            find_longest(next_node, end, graph, seen, steps + weight, max_steps);
        }
    }
    seen.remove(&node);
}

fn fill_graph(
    start: i32,
    nodes: &HashSet<i32>,
    grid: &Vec<Vec<char>>,
    graph: &mut HashMap<i32, Vec<(i32, usize)>>,
) {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut queue: VecDeque<(i32, HashSet<i32>)> = VecDeque::new();
    queue.push_back((start, HashSet::new()));

    while let Some((index, mut path)) = queue.pop_front() {
        if index != start && nodes.contains(&index) {
            graph.entry(start).or_default().push((index, path.len()));
            continue;
        }

        path.insert(index);

        let (x, y) = (index % width, index / height);
        match grid[y as usize][x as usize] {
            '.' | '^' | 'v' | '>' | '<' => {
                for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                    if (0..width).contains(&nx)
                        && (0..height).contains(&ny)
                        && grid[ny as usize][nx as usize] != '#'
                        && !path.contains(&(nx + ny * height))
                    {
                        queue.push_back(((nx + ny * height), path.clone()));
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
