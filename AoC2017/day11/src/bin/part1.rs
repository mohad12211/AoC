fn main() {
    let input = include_str!("input.test.txt");
    // let input = include_str!("input.txt");

    let (mut x, mut y): (i32, i32) = (0, 0);

    for step in input.trim().split(',') {
        match step {
            "n" => y += 2,
            "s" => y -= 2,
            "ne" => {
                y += 1;
                x += 1;
            }
            "nw" => {
                y += 1;
                x -= 1;
            }
            "se" => {
                y -= 1;
                x += 1;
            }
            "sw" => {
                y -= 1;
                x -= 1;
            }
            _ => unreachable!(),
        };
    }

    let (x, y) = (x.abs(), y.abs());
    if y > x {
        println!("{}", x + ((y - x) / 2));
    } else if y <= x {
        println!("{}", x);
    }
}
