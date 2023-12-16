use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut max = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let cond_val: i64 = parts[6].parse().unwrap();
        let cond_reg = registers.get(parts[4]).copied().unwrap_or_default();
        match parts[5] {
            ">" => {
                if !(cond_reg > cond_val) {
                    continue;
                }
            }
            "<" => {
                if !(cond_reg < cond_val) {
                    continue;
                }
            }
            ">=" => {
                if !(cond_reg >= cond_val) {
                    continue;
                }
            }
            "<=" => {
                if !(cond_reg <= cond_val) {
                    continue;
                }
            }
            "==" => {
                if !(cond_reg == cond_val) {
                    continue;
                }
            }
            "!=" => {
                if !(cond_reg != cond_val) {
                    continue;
                }
            }
            _ => unreachable!(),
        };
        let diff: i64 = parts[2].parse().unwrap();
        match parts[1] {
            "inc" => {
                registers
                    .entry(parts[0])
                    .and_modify(|v| *v += diff)
                    .or_insert(diff);
            }
            "dec" => {
                registers
                    .entry(parts[0])
                    .and_modify(|v| *v -= diff)
                    .or_insert(-diff);
            }
            _ => unreachable!(),
        };
        max = max.max(registers.values().max().copied().unwrap());
    }

    println!("{max}");
}
