use std::collections::{HashMap, HashSet};

fn get_closest_point(
    points: &HashSet<(usize, usize)>,
    (x, y): (usize, usize),
) -> Option<(usize, usize)> {
    let mut distances: Vec<_> = points
        .iter()
        .map(|&(px, py)| ((x.abs_diff(px) + y.abs_diff(py)), (px, py)))
        .collect();
    distances.sort_by_key(|(dist, _point)| *dist);

    if distances[0].0 != distances[1].0 {
        Some(distances[0].1)
    } else {
        None
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let points: HashSet<(usize, usize)> = input
        .lines()
        .map(|l| l.split_once(", ").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let mut finite_points: HashSet<(usize, usize)> = points.clone();

    let min_x = *points.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap() + 1;

    let min_y = *points.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap() + 1;

    for x in min_x..=max_x {
        if let Some(p) = get_closest_point(&points, (x, min_y)) {
            finite_points.remove(&p);
        }
        if let Some(p) = get_closest_point(&points, (x, max_y)) {
            finite_points.remove(&p);
        }
    }

    for y in min_y..=max_y {
        if let Some(p) = get_closest_point(&points, (min_x, y)) {
            finite_points.remove(&p);
        }
        if let Some(p) = get_closest_point(&points, (max_x, y)) {
            finite_points.remove(&p);
        }
    }

    let mut finite_points_area: HashMap<(usize, usize), usize> =
        finite_points.iter().map(|x| (*x, 0)).collect();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(closest_point) = get_closest_point(&points, (x, y)) {
                finite_points_area
                    .entry(closest_point)
                    .and_modify(|n| *n += 1);
            }
        }
    }

    println!("{:?}", finite_points_area.values().max().unwrap());
}
