use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operand {
    Reg(usize),
    Val(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ins {
    Snd(Operand),
    Set(usize, Operand),
    Add(usize, Operand),
    Mul(usize, Operand),
    Mod(usize, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

fn parse_reg(s: &str) -> usize {
    (s.as_bytes()[0] - b'a') as usize
}

fn parse_operand(s: &str) -> Operand {
    match s.parse::<isize>() {
        Ok(val) => Operand::Val(val),
        Err(_) => Operand::Reg(parse_reg(s)),
    }
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, args) = s.split_once(' ').ok_or("Expected space")?;
        Ok(match ins {
            "snd" => Ins::Snd(parse_operand(args)),
            "set" => {
                let (dst, src) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Set(parse_reg(dst), parse_operand(src))
            }
            "add" => {
                let (x, y) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Add(parse_reg(x), parse_operand(y))
            }
            "mul" => {
                let (x, y) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Mul(parse_reg(x), parse_operand(y))
            }
            "mod" => {
                let (x, y) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Mod(parse_reg(x), parse_operand(y))
            }
            "rcv" => Ins::Rcv(parse_operand(args)),
            "jgz" => {
                let (cond, offset) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Jgz(parse_operand(cond), parse_operand(offset))
            }
            _ => unreachable!(),
        })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut regs = [0; 26];
    let mut ip = 0;
    let mut last_played_freq = 0;
    let instructions: Vec<Ins> = input.lines().map(|l| Ins::from_str(l).unwrap()).collect();

    while let Some(&instruction) = instructions.get(ip) {
        match instruction {
            Ins::Snd(x) => {
                let freq = match x {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                last_played_freq = freq;
            }
            Ins::Set(dst, src) => {
                let src_val = match src {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                regs[dst] = src_val;
            }
            Ins::Add(x, y) => {
                let y_val = match y {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                regs[x] += y_val;
            }
            Ins::Mul(x, y) => {
                let y_val = match y {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                regs[x] *= y_val;
            }
            Ins::Mod(x, y) => {
                let y_val = match y {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                regs[x] %= y_val;
            }
            Ins::Rcv(x) => {
                let x_val = match x {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                if x_val != 0 {
                    println!("{last_played_freq}");
                    break;
                }
            }
            Ins::Jgz(cond, offset) => {
                let cond = match cond {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                let offset = match offset {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };

                if cond > 0 {
                    ip = ip.wrapping_add_signed(offset);
                    continue;
                }
            }
        }
        ip += 1;
    }
}
