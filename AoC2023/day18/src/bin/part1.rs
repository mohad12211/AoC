fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let (mut x, mut y) = (0, 0);
    let mut vertices = vec![];
    let mut boundry_count = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let steps: i64 = parts[1].parse().unwrap();
        match parts[0] {
            "R" => {
                x += steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            "L" => {
                x -= steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            "U" => {
                y -= steps;
                boundry_count += steps;
                vertices.push((x, y));
            }
            "D" => {
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
