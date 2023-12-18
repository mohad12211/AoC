fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let (mut x, mut y) = (0, 0);
    let mut vertices = vec![];
    let mut boundry_count = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut hex = parts[2]
            .strip_prefix("(#")
            .and_then(|s| s.strip_suffix(')'))
            .unwrap()
            .to_string();
        let dir = hex.pop().unwrap();
        let steps = i64::from_str_radix(&hex, 16).unwrap();
        match dir {
            '0' => {
                x += steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            '2' => {
                x -= steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            '3' => {
                y -= steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            '1' => {
                y += steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            _ => unreachable!(),
        };
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula
    let area = vertices
        .windows(2)
        .map(|p| (p[0], p[1]))
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2))
        .sum::<i64>()
        / 2;
    // https://en.wikipedia.org/wiki/Pick%27s_theorem#Formula
    let interior = area - (boundry_count / 2) + 1;
    println!("{}", interior + boundry_count);
}
