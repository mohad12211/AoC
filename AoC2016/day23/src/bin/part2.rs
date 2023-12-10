use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operand {
    Reg(usize),
    Val(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ins {
    Cpy(Operand, Operand),
    Inc(usize),
    Dec(usize),
    Jnz(Operand, Operand),
    Tgl(usize),
}

impl Ins {
    fn toggle(&self) -> Ins {
        match *self {
            Ins::Cpy(src, dst) => Ins::Jnz(src, dst),
            Ins::Inc(reg) => Ins::Dec(reg),
            Ins::Dec(reg) => Ins::Inc(reg),
            Ins::Jnz(cond, offset) => Ins::Cpy(cond, offset),
            Ins::Tgl(reg) => Ins::Inc(reg),
        }
    }
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
        let (ins, args) = s.split_once(' ').ok_or("invalid instruction")?;
        Ok(match ins {
            "cpy" => {
                let (src, dst) = args.split_once(' ').ok_or("invalid cpy instruction")?;
                Ins::Cpy(parse_operand(src), parse_operand(dst))
            }
            "inc" => Ins::Inc(parse_reg(args)),
            "dec" => Ins::Dec(parse_reg(args)),
            "jnz" => {
                let (cond, offset) = args.split_once(' ').ok_or("invalid jnz instruction")?;
                Ins::Jnz(parse_operand(cond), parse_operand(offset))
            }
            "tgl" => Ins::Tgl(parse_reg(args)),
            _ => unreachable!(),
        })
    }
}

// TODO: optimize this according to the hint given in the problem
fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut instructions: Vec<_> = input.lines().map(|l| Ins::from_str(l).unwrap()).collect();

    let mut regs = [12, 0, 0, 0];
    let mut ip = 0;

    while let Some(&instruction) = instructions.get(ip) {
        match instruction {
            Ins::Cpy(src, dst) => 'cpy: {
                let src = match src {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                let dst = match dst {
                    Operand::Reg(r) => r,
                    Operand::Val(_) => {
                        break 'cpy;
                    }
                };
                regs[dst] = src;
            }
            Ins::Inc(reg) => {
                regs[reg] += 1;
            }
            Ins::Dec(reg) => {
                regs[reg] -= 1;
            }
            Ins::Jnz(cond, offset) => {
                let cond = match cond {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                let offset = match offset {
                    Operand::Reg(reg) => regs[reg],
                    Operand::Val(val) => val,
                };
                if cond != 0 {
                    ip = ip.wrapping_add_signed(offset);
                    continue;
                }
            }
            Ins::Tgl(reg) => {
                let offset = regs[reg];
                let instruction_index = ip.checked_add_signed(offset).expect("Shouldn't overflow");
                if let Some(instruction) = instructions.get_mut(instruction_index) {
                    *instruction = instruction.toggle();
                }
            }
        }

        ip += 1;
    }

    println!("{}", regs[0]);
}
