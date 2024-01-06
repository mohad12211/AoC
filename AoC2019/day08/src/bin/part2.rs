fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let rows = 6;
    let columns = 25;

    let digits: Vec<_> = input.trim().chars().collect();
    let mut image: Vec<_> = vec!['2'; columns * rows];
    for layer in digits.chunks(columns * rows) {
        for (i, &pixel) in layer.iter().enumerate() {
            if image[i] == '2' {
                image[i] = pixel;
            }
        }
    }

    for row in 0..rows {
        for col in 0..columns {
            if image[row * columns + col] == '1' {
                print!("█")
            } else if image[row * columns + col] == '0' {
                print!(".")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}
