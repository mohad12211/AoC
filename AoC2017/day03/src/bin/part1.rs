fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let num: i64 = input.trim().parse().unwrap();
    let (x, y) = get_pos(num);
    println!("{}", x.abs() + y.abs());
}

fn get_pos(num: i64) -> (i64, i64) {
    let mut i = 0;
    while (i * 2 + 1 as i64).pow(2) < num {
        i += 1;
    }
    let mut corner = (i * 2 + 1 as i64).pow(2);
    if num <= corner && num > corner - 2 * i {
        return (i - (corner - num), -i);
    }
    corner -= 2 * i;
    if num <= corner && num > corner - 2 * i {
        return (-i, i - (corner - num));
    }
    corner -= 2 * i;
    if num <= corner && num > corner - 2 * i {
        return ((corner - num) - i, i);
    }
    corner -= 2 * i;
    if num <= corner && num > corner - 2 * i {
        return (i, i - (corner - num));
    }
    unreachable!()
}
