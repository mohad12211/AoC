#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

const input: &str = include_str!("../../inputs/input.txt");

#[derive(Clone, Copy, Debug)]
enum Ins {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmp(isize),
    jie(usize, isize),
    jio(usize, isize),
}

fn parse_reg(s: Option<&str>) -> usize {
    (s.unwrap().as_bytes()[0] - b'a') as usize
}

fn main() {
    // regs [a, b, ip]
    let mut regs: [usize; 3] = [1, 0, 0];
    let mut ins = vec![];

    for line in input.lines() {
        let mut parts = line.split(" ");
        ins.push(match parts.next().unwrap() {
            "hlf" => Ins::hlf(parse_reg(parts.next())),
            "tpl" => Ins::tpl(parse_reg(parts.next())),
            "inc" => Ins::inc(parse_reg(parts.next())),
            "jmp" => Ins::jmp(parts.next().unwrap().parse().unwrap()),
            "jie" => Ins::jie(
                parse_reg(parts.next()),
                parts.next().unwrap().parse().unwrap(),
            ),
            "jio" => Ins::jio(
                parse_reg(parts.next()),
                parts.next().unwrap().parse().unwrap(),
            ),
            _ => unreachable!(),
        });
    }

    loop {
        let Some(instruction) = ins.get(regs[2]).copied() else {
            break;
        };
        match instruction {
            Ins::hlf(reg) => {
                regs[reg] /= 2;
                regs[2] += 1;
            }
            Ins::tpl(reg) => {
                regs[reg] *= 3;
                regs[2] += 1;
            }
            Ins::inc(reg) => {
                regs[reg] += 1;
                regs[2] += 1;
            }
            Ins::jmp(offset) => {
                regs[2] = regs[2].checked_add_signed(offset).unwrap();
            }
            Ins::jie(reg, offset) => {
                if regs[reg] % 2 == 0 {
                    regs[2] = regs[2].checked_add_signed(offset).unwrap();
                } else {
                    regs[2] += 1;
                }
            }
            Ins::jio(reg, offset) => {
                if regs[reg] == 1 {
                    regs[2] = regs[2].checked_add_signed(offset).unwrap();
                } else {
                    regs[2] += 1;
                }
            }
        }
    }

    println!("{}", regs[1]);
}
