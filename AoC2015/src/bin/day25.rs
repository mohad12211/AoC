fn main() {
    let req_row = 2981;
    let req_col = 3075;
    let ith = { (1..req_row).sum::<i64>() + 1 + (req_row + 1..req_row + req_col).sum::<i64>() };

    let mut prev: i64 = 20151125;
    let mul = 252533;
    let div = 33554393;

    for _ in 2..=ith {
        prev = (prev * mul) % div;
    }

    println!("{prev}");
}
