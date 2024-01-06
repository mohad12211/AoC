fn main() {
    // let mut lines = include_str!("input.test.txt").lines();
    let mut lines = include_str!("input.txt").lines();

    let (mut cx, mut cy) = (0, 0);
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
        ranges.push(((cx.min(px), cx.max(px)), (cy.min(py), cy.max(py))));
    }

    (cx, cy) = (0, 0);
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
        let ((xs1, xe1), (ys1, ye1)) = ((cx.min(px), cx.max(px)), (cy.min(py), cy.max(py)));
        for &((xs2, xe2), (ys2, ye2)) in &ranges {
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
                if x != 0 || y != 0 {
                    min = min.min(x.abs() + y.abs());
                }
            }
        }
    }
    println!("{min}");
}
