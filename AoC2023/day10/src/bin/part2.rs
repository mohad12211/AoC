use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Copy)]
struct Tile {
    x: usize,
    y: usize,
    prev_x: usize,
    prev_y: usize,
}

struct Grid {
    tiles: Vec<Vec<char>>,
    loop_path: HashSet<(usize, usize)>,
    enclosed: HashSet<(usize, usize)>,
    first_step: Tile,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let (start_x, start_y) = tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
            .ok_or("should have start 'S'")?;
        let (max_x, max_y) = (tiles[0].len() - 1, tiles.len() - 1);
        let (mut first_step_x, mut first_step_y) = (0, 0);
        let mut directions = vec!['|', '-', 'J', 'F', '7', 'L'];
        // can go right
        if start_x < max_x && matches!(tiles[start_y][start_x + 1], '-' | '7' | 'J') {
            (first_step_x, first_step_y) = (start_x + 1, start_y);
            // keep only directions that go right
            directions.retain(|&d| matches!(d, '-' | 'F' | 'L'));
        }
        // can go left
        if start_x > 0 && matches!(tiles[start_y][start_x - 1], '-' | 'F' | 'L') {
            (first_step_x, first_step_y) = (start_x - 1, start_y);
            // keep only directions that go left
            directions.retain(|&d| matches!(d, '-' | '7' | 'J'));
        }
        // can go up
        if start_y > 0 && matches!(tiles[start_y - 1][start_x], '|' | '7' | 'F') {
            (first_step_x, first_step_y) = (start_x, start_y - 1);
            // keep only directions that go up
            directions.retain(|&d| matches!(d, '|' | 'L' | 'J'));
        }
        // can go down
        if start_y < max_y && matches!(tiles[start_y + 1][start_x], '|' | 'L' | 'J') {
            (first_step_x, first_step_y) = (start_x, start_y + 1);
            // keep only directions that go down
            directions.retain(|&d| matches!(d, '|' | '7' | 'F'));
        }
        assert_eq!(directions.len(), 1);
        assert_ne!(first_step_x * first_step_y, 0);
        tiles[start_y][start_x] = *directions.first().ok_or("should have one result")?;
        Ok(Self {
            tiles,
            loop_path: HashSet::new(),
            enclosed: HashSet::new(),
            first_step: Tile {
                x: first_step_x,
                y: first_step_y,
                prev_x: start_x,
                prev_y: start_y,
            },
        })
    }
}

impl Grid {
    fn find_loop(&mut self, tile: Tile) {
        self.loop_path.insert((tile.x, tile.y));
        if tile.x == self.first_step.prev_x && tile.y == self.first_step.prev_y {
            return;
        }
        self.find_loop(self.next_tile(tile))
    }

    fn next_tile(&self, tile: Tile) -> Tile {
        let dx = tile.x as isize - tile.prev_x as isize;
        let dy = tile.y as isize - tile.prev_y as isize;
        let (next_x, next_y) = match self.tiles[tile.y][tile.x] {
            '|' | '-' => (
                tile.x.saturating_add_signed(dx),
                tile.y.saturating_add_signed(dy),
            ),
            'L' | '7' => (
                tile.x.saturating_add_signed(dy),
                tile.y.saturating_add_signed(dx),
            ),
            'J' | 'F' => (
                tile.x.saturating_add_signed(-dy),
                tile.y.saturating_add_signed(-dx),
            ),
            _ => unreachable!(),
        };
        Tile {
            x: next_x,
            y: next_y,
            prev_x: tile.x,
            prev_y: tile.y,
        }
    }

    fn find_enclosed(&mut self) {
        for (y, row) in self.tiles.iter().enumerate() {
            let mut enclosed_parity = false;
            for (x, char) in row.iter().enumerate() {
                if self.loop_path.contains(&(x, y)) {
                    // or can be: '|' , '7' , 'F'
                    if matches!(char, '|' | 'J' | 'L') {
                        enclosed_parity = !enclosed_parity;
                    }
                } else if enclosed_parity {
                    self.enclosed.insert((x, y));
                }
            }
        }
    }

    fn draw(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if self.loop_path.contains(&(x, y)) {
                    let box_drawing_char = match char {
                        '|' => '│',
                        '-' => '─',
                        'L' => '└',
                        '7' => '┐',
                        'J' => '┘',
                        'F' => '┌',
                        'S' => 'S',
                        _ => unreachable!(),
                    };
                    print!("{box_drawing_char}");
                } else if self.enclosed.contains(&(x, y)) {
                    print!("I");
                } else {
                    print!("O");
                }
            }
            println!()
        }
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut grid: Grid = input.parse().unwrap();
    grid.find_loop(grid.first_step);
    grid.find_enclosed();
    grid.draw();
    println!("{}", grid.enclosed.len());
}
