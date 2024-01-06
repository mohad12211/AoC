fn main() {
    // let mut lines = include_str!("input.test.txt").lines();
    let mut lines = include_str!("input.txt").lines();

    let (mut cx, mut cy) = (0, 0);
    let mut steps = 0;
    let mut ranges = Vec::new();
    for step in lines.next().unwrap().split(',') {
        let dir = step[0..1].as_bytes()[0];
        let num: i64 = step[1..].parse().unwrap();
        let (px, py) = (cx, cy);
        match dir {
            b'U' => cy += num,
            b'D' => cy -= num,
            b'R' => cx += num,
            b'L' => cx -= num,
            _ => unreachable!(),
        };
        ranges.push(((px, cx), (py, cy), steps));
        steps += num;
    }

    (cx, cy) = (0, 0);
    steps = 0;
    let mut min = i64::MAX;
    for step in lines.next().unwrap().split(',') {
        let dir = step[0..1].as_bytes()[0];
        let num: i64 = step[1..].parse().unwrap();
        let (px, py) = (cx, cy);
        match dir {
            b'U' => cy += num,
            b'D' => cy -= num,
            b'R' => cx += num,
            b'L' => cx -= num,
            _ => unreachable!(),
        };
        let ((xs1, xe1), (ys1, ye1)) = ((px, cx), (py, cy));
        for &((xs2, xe2), (ys2, ye2), steps2) in &ranges {
            if let Some((x, y)) = get_intersection((xs1, xe1, ys1, ye1, xs2, xe2, ys2, ye2)) {
                let (dx1, dy1) = (x - xs1, y - ys1);
                let (dx2, dy2) = (x - xs2, y - ys2);
                let steps1 = steps + dx1.abs() + dy1.abs();
                let steps2 = steps2 + dx2.abs() + dy2.abs();
                if steps1 != 0 || steps2 != 0 {
                    min = min.min(steps1 + steps2);
                }
            }
        }
        steps += num;
    }
    println!("{min}");
}

fn get_intersection(
    (xs1, xe1, ys1, ye1, xs2, xe2, ys2, ye2): (i64, i64, i64, i64, i64, i64, i64, i64),
) -> Option<(i64, i64)> {
    let (xs1, xe1) = (xs1.min(xe1), xs1.max(xe1));
    let (xs2, xe2) = (xs2.min(xe2), xs2.max(xe2));
    let (ys1, ye1) = (ys1.min(ye1), ys1.max(ye1));
    let (ys2, ye2) = (ys2.min(ye2), ys2.max(ye2));
    if xs1.max(xs2) <= xe1.min(xe2) && ys1.max(ys2) <= ye1.min(ye2) {
        let x = if xs1 == xe1 {
            xs1
        } else {
            /*if xs2 == xe2*/
            xs2
        };
        let y = if ys1 == ye1 {
            ys1
        } else {
            // if ys2 == ye2
            ys2
        };
        return Some((x, y));
    }
    None
}
