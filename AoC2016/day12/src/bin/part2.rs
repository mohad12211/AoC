use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Reg(usize),
    Val(isize),
}

#[derive(Debug, Clone, Copy)]
enum Ins {
    Cpy(Operand, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Operand, isize),
}

fn parse_reg(s: &str) -> usize {
    (s.as_bytes()[0] - b'a') as usize
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, args) = s.split_once(' ').ok_or("invalid instruction")?;
        Ok(match ins {
            "cpy" => {
                let (src, dst) = args.split_once(' ').ok_or("invalid cpy instruction")?;
                let src = match src.parse::<isize>() {
                    Ok(v) => Operand::Val(v),
                    Err(_) => Operand::Reg(parse_reg(src)),
                };
                let dst = parse_reg(dst);
                Ins::Cpy(src, dst)
            }
            "inc" => Ins::Inc(parse_reg(args)),
            "dec" => Ins::Dec(parse_reg(args)),
            "jnz" => {
                let (cond, offset) = args.split_once(' ').ok_or("invalid jnz instruction")?;
                let cond = match cond.parse::<isize>() {
                    Ok(v) => Operand::Val(v),
                    Err(_) => Operand::Reg(parse_reg(cond)),
                };
                Ins::Jnz(cond, offset.parse().map_err(|_| "invalid jnz operand")?)
            }
            _ => unreachable!(),
        })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let instructions: Vec<_> = input.lines().map(|l| Ins::from_str(l).unwrap()).collect();

    let mut regs = [0, 0, 1, 0];
    let mut ip = 0;

    while let Some(&instruction) = instructions.get(ip) {
        match instruction {
            Ins::Cpy(src, dst) => {
                let value = match src {
                    Operand::Reg(r) => regs[r],
                    Operand::Val(v) => v,
                };
                regs[dst] = value;
                ip += 1;
            }
            Ins::Inc(reg) => {
                regs[reg] += 1;
                ip += 1;
            }
            Ins::Dec(reg) => {
                regs[reg] -= 1;
                ip += 1;
            }
            Ins::Jnz(cond, offset) => {
                let value = match cond {
                    Operand::Reg(r) => regs[r],
                    Operand::Val(v) => v,
                };
                if value != 0 {
                    ip = ip.checked_add_signed(offset).expect("Shouldn't overflow");
                } else {
                    ip += 1;
                }
            }
        }
    }

    println!("{}", regs[0]);
}
