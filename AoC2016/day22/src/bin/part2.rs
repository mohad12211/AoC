use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

struct Node {
    point: (usize, usize),
    size: usize,
    used: usize,
}

struct Solver {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(Self {
            point: parts[0]
                .strip_prefix("/dev/grid/node-x")
                .and_then(|s| s.split_once("-y"))
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .ok_or("invalid point")?,
            size: parts[1]
                .strip_suffix('T')
                .and_then(|s| s.parse().ok())
                .ok_or("invalid size")?,
            used: parts[2]
                .strip_suffix('T')
                .and_then(|s| s.parse().ok())
                .ok_or("invalid used")?,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let nodes: Vec<_> = input
        .lines()
        .skip(2)
        .map(|line| Node::from_str(line).unwrap())
        .collect();
    let (max_x, max_y) = nodes.iter().map(|node| node.point).max().unwrap();

    let solver = Solver {
        start: nodes
            .iter()
            .find(|node| node.used == 0)
            .map(|n| n.point)
            .unwrap(),
        end: (max_x - 1, 0),
        nodes,
        width: max_x + 1,
        height: max_y + 1,
    };

    if let Some(steps) = solver.solve() {
        println!("{}", steps);
    } else {
        println!("No path found.");
    }
}

impl Solver {
    fn bfs(&self) -> Option<usize> {
        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        queue.push_back((self.start, 0));

        while let Some((point, steps)) = queue.pop_front() {
            if visited.contains(&point) {
                continue;
            }
            if point == self.end {
                return Some(steps);
            }
            for neighbour in self.get_neighbors(point) {
                queue.push_back((neighbour, steps + 1));
            }
            visited.insert(point);
        }

        None
    }

    fn get_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        neighbors
            .into_iter()
            .filter(|n| !self.is_wall(*n))
            .collect()
    }

    // treat full nodes as wall
    fn is_wall(&self, (x, y): (usize, usize)) -> bool {
        self.nodes[x * self.height + y].used > self.nodes[0].size
    }

    fn solve(&self) -> Option<usize> {
        // Steps from _ to left of G (from bfs), then + 1 to go right and move G to the left,
        // then we want G keep moving left width-1 times, we already did one,
        // so we need width-2 more, each "G move" is 5 steps
        self.bfs().map(|n| n + 1 + (self.width - 2) * 5)
    }
}
