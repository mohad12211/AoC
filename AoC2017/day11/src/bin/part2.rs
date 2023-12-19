fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut max_hex_steps = 0;

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
        max_hex_steps = max_hex_steps.max(to_hex((x, y)));
    }

    println!("{max_hex_steps}");
}

fn to_hex((x, y): (i32, i32)) -> i32 {
    if y > x {
        x + ((y - x) / 2)
    } else if y <= x {
        x
    } else {
        unreachable!()
    }
}
