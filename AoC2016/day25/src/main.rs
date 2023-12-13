use std::{collections::HashSet, str::FromStr};
use vm::Ins;

mod vm;

fn main() {
    let input = include_str!("input.txt");

    let instructions: Vec<_> = input.lines().map(|l| Ins::from_str(l).unwrap()).collect();
    for a in 0.. {
        let mut regs = [a, 0, 0, 0];
        let mut ip = 0;
        let mut prev_out = None;
        let mut set = HashSet::new();
        loop {
            if let Some(out) = vm::execute(&instructions, &mut regs, &mut ip) {
                if let Some(prev_out) = prev_out {
                    if (prev_out == 1 && out != 0) || (prev_out == 0 && out != 1) {
                        break;
                    }
                } else if prev_out.is_none() && out != 0 {
                    // pattern should start at 0
                    break;
                }
                prev_out = Some(out);
                // if we have the same register + ip state, then we have an infinite loop
                if set.contains(&(regs, ip)) {
                    println!("{a}");
                    return;
                } else {
                    set.insert((regs, ip));
                }
            }
        }
    }
}
