use std::collections::VecDeque;

// by investigating `show_until(250);` we see that the pattern is as follows:
//
// n = 1 and n = 2 are trivial
//
// for n where n is a power of 3, f(n) = n^3;
//
// otherwise: let the previous power of 3 be x
// for n where: 2x >= n > x, f(n) will start from 1 and increase by 1
// so f(n) = n - x
//
// for n where: 3x > n > 2x, f(n) will continue increasing by 2 instead
// so f(n) = (n - x) + (n - 2x) = 2n - 3x
//
fn show_until(until: u32) {
    let mut v = Vec::new();
    for i in 1..=until {
        v.push(i);

        let mut elves = VecDeque::from(v.clone());
        while elves.len() > 1 {
            // will round down (i.e. choose the left one)
            let opposite = elves.len() / 2;

            elves.remove(opposite);

            // rotate the circle to the next elf
            let first = elves.pop_front().unwrap();
            elves.push_back(first);
        }

        println!("n: {i} => f(n): {}", elves[0]);
    }
}

fn main() {
    // show_until(250);
    let n: u64 = 3005290;
    let mut x = 1;
    let result;

    while x * 3 <= n {
        x *= 3;
    }

    if x == n {
        result = x;
    } else if n <= 2 * x {
        result = n - x;
    } else {
        result = 2 * n - 3 * x;
    }
    println!("{result}");
}
