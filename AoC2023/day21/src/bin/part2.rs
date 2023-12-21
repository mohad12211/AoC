use std::collections::{HashSet, VecDeque};

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let grid_width = map[0].len() as i64;
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, e)| (*e == 'S').then_some(x as i64))
                .map(|x| (x, y as i64))
        })
        .unwrap();

    // call the function we wrote in p1 f(N), the input to this function is going outward N steps,
    // the output is proportional the to area, so the function must be quadratic,
    // now we want to calculate f(26501365), which is f(202300 * 131 + 65)
    // (obviously specifically tailored to the input),
    // define g(x) = f(x * 131 + 65), calculate g(0), g(1), and g(2),
    // you will notice that the second order delta is constant (since it's a quadratic),
    // from there, we can calculate g(202300) in a for loop,
    // given the first order delta and the second order delta (similar to day 9)

    // steps = 2023 * 131 + 65
    let steps = 26501365;
    // [ f(0), f(1), f(2) ]
    let f012: Vec<_> = (0..3).map(|i| bfs(65 + 131 * i, start, &map)).collect();
    // [ f(1) - f(0), f(2) - f(1) ]
    let first_order_deltas: Vec<_> = f012.windows(2).map(|x| x[1] - x[0]).collect();
    let second_order_delta = first_order_deltas[1] - first_order_deltas[0];
    let grids_to_visit = steps / grid_width;
    let mut prev_first_order_delta = first_order_deltas[0];
    let mut prev_value = f012[0];

    for _ in 0..(grids_to_visit) {
        prev_first_order_delta += second_order_delta;
        prev_value += prev_first_order_delta;
    }

    println!("{prev_value}");
}

fn bfs(max_steps: i64, start: (i64, i64), map: &Vec<Vec<char>>) -> usize {
    let width = map[0].len() as i64;
    let height = map.len() as i64;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    let mut end_positions = 0;

    while let Some(((x, y), steps)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        if steps % 2 == max_steps % 2 {
            end_positions += 1;
        }
        visited.insert((x, y));
        if steps == max_steps {
            continue;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = ((x + dx), (y + dy));
            let (inx, iny) = (nx.rem_euclid(width), ny.rem_euclid(height));
            if map[iny as usize][inx as usize] != '#' {
                queue.push_back(((nx, ny), steps + 1));
            }
        }
    }

    end_positions
}
