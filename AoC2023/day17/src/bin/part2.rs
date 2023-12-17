use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
    dir_steps: u32,
}

fn get_neighbours(
    State {
        pos: (x, y),
        dir: (dx, dy),
        dir_steps,
    }: State,
    grid: &Vec<Vec<u32>>,
) -> Vec<State> {
    let mut neighbours = Vec::new();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    if dir_steps < 10 && (dx, dy) != (0, 0) {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            neighbours.push(State {
                pos: (nx, ny),
                dir: (dx, dy),
                dir_steps: dir_steps + 1,
            });
        }
    }

    if dir_steps >= 4 || (dx, dy) == (0, 0) {
        for (ndx, ndy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if (ndx, ndy) != (dx, dy) && (ndx, ndy) != (-dx, -dy) {
                let nx = x + ndx;
                let ny = y + ndy;
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    neighbours.push(State {
                        pos: (nx, ny),
                        dir: (ndx, ndy),
                        dir_steps: 1,
                    });
                }
            }
        }
    }
    neighbours
}

fn dijkstra(grid: &Vec<Vec<u32>>, start: (i32, i32), finish: (i32, i32)) -> Option<u32> {
    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    let start_state = State {
        pos: start,
        dir: (0, 0),
        dir_steps: 0,
    };

    costs.insert(start_state, 0);
    heap.push(Reverse((0, start_state)));

    while let Some(Reverse((cost, state))) = heap.pop() {
        if state.pos == finish && state.dir_steps >= 4 {
            return Some(cost);
        }

        if cost > *costs.get(&state).unwrap_or(&u32::MAX) {
            continue;
        }

        for neighbour in get_neighbours(state, grid) {
            let (nx, ny) = neighbour.pos;
            let next_cost = cost + grid[ny as usize][nx as usize];
            if next_cost < *costs.get(&neighbour).unwrap_or(&u32::MAX) {
                costs.insert(neighbour, next_cost);
                heap.push(Reverse((next_cost, neighbour)));
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let start = (0, 0);
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let end = (width - 1, height - 1);

    if let Some(least_cost) = dijkstra(&grid, start, end) {
        println!("{least_cost}");
    } else {
        println!("No Path Found");
    }
}
