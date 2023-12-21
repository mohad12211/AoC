use std::{collections::VecDeque, str::FromStr};

// interesting read
// https://www.reddit.com/r/adventofcode/comments/7kjaiu/2017_day_18_what_is_the_duet_actually_doing/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Register(usize),
    Const(isize),
}

impl Value {
    fn extract_value(&self, regs: &[isize]) -> isize {
        match *self {
            Value::Register(reg) => regs[reg],
            Value::Const(val) => val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ins {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}

fn parse_register(s: &str) -> usize {
    (s.as_bytes()[0] - b'a') as usize
}

fn parse_value(s: &str) -> Value {
    match s.parse::<isize>() {
        Ok(val) => Value::Const(val),
        Err(_) => Value::Register(parse_register(s)),
    }
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, args) = s.split_once(' ').ok_or("Expected space")?;
        Ok(match ins {
            "snd" => Ins::Snd(parse_value(args)),
            "set" => {
                let (dst, src) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Set(parse_register(dst), parse_value(src))
            }
            "add" => {
                let (x, y) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Add(parse_register(x), parse_value(y))
            }
            "mul" => {
                let (x, y) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Mul(parse_register(x), parse_value(y))
            }
            "mod" => {
                let (x, y) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Mod(parse_register(x), parse_value(y))
            }
            "rcv" => Ins::Rcv(parse_register(args)),
            "jgz" => {
                let (cond, offset) = args.split_once(' ').ok_or("Expected space")?;
                Ins::Jgz(parse_value(cond), parse_value(offset))
            }
            _ => unreachable!(),
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
enum InsResult {
    Next,
    Jump,
    Blocked,
    Terminated,
    Sending(isize),
}

struct Program<'a> {
    queue: VecDeque<isize>,
    is_blocked: bool,
    regs: [isize; 26],
    ip: usize,
    instructions: &'a Vec<Ins>,
}

impl<'a> Program<'a> {
    fn execute(&mut self) -> InsResult {
        let Some(&instruction) = self.instructions.get(self.ip) else {
            return InsResult::Terminated;
        };
        let result = match instruction {
            Ins::Snd(x) => InsResult::Sending(x.extract_value(&self.regs)),
            Ins::Set(dst, src) => {
                self.regs[dst] = src.extract_value(&self.regs);
                InsResult::Next
            }
            Ins::Add(x, y) => {
                self.regs[x] += y.extract_value(&self.regs);
                InsResult::Next
            }
            Ins::Mul(x, y) => {
                self.regs[x] *= y.extract_value(&self.regs);
                InsResult::Next
            }
            Ins::Mod(x, y) => {
                self.regs[x] %= y.extract_value(&self.regs);
                InsResult::Next
            }
            Ins::Rcv(x) => {
                if let Some(rcved) = self.queue.pop_front() {
                    self.regs[x] = rcved;
                    self.is_blocked = false;
                    InsResult::Next
                } else {
                    self.is_blocked = true;
                    InsResult::Blocked
                }
            }
            Ins::Jgz(cond, offset) => {
                if cond.extract_value(&self.regs) > 0 {
                    self.ip = self
                        .ip
                        .saturating_add_signed(offset.extract_value(&self.regs));
                    InsResult::Jump
                } else {
                    InsResult::Next
                }
            }
        };
        if matches!(result, InsResult::Next | InsResult::Sending(_)) {
            self.ip += 1;
        }
        result
    }

    fn push_message(&mut self, x: isize) {
        self.queue.push_back(x);
        if self.is_blocked {
            self.execute();
        }
    }
}

struct Cpu<'a> {
    p0: Program<'a>,
    p1: Program<'a>,
    p1_message_count: usize,
}

impl<'a> Cpu<'a> {
    fn run(&mut self) {
        loop {
            let p0_result = self.p0.execute();
            let p0_terminated = match p0_result {
                InsResult::Next | InsResult::Jump => false,
                InsResult::Blocked | InsResult::Terminated => true,
                InsResult::Sending(x) => {
                    self.p1.push_message(x);
                    false
                }
            };

            let p1_result = self.p1.execute();
            let p1_terminated = match p1_result {
                InsResult::Next | InsResult::Jump => false,
                InsResult::Blocked | InsResult::Terminated => true,
                InsResult::Sending(x) => {
                    self.p1_message_count += 1;
                    self.p0.push_message(x);
                    false
                }
            };

            if p0_terminated && p1_terminated {
                break;
            }
        }

        println!("{}", self.p1_message_count);
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let instructions: &Vec<Ins> = &input.lines().map(|l| Ins::from_str(l).unwrap()).collect();
    let p0 = Program {
        ip: 0,
        is_blocked: false,
        queue: VecDeque::new(),
        regs: [0; 26],
        instructions,
    };
    let mut p1_regs = [0; 26];
    p1_regs[parse_register("p")] = 1;
    let p1 = Program {
        ip: 0,
        is_blocked: false,
        queue: VecDeque::new(),
        regs: p1_regs,
        instructions,
    };
    let mut cpu = Cpu {
        p0,
        p1,
        p1_message_count: 0,
    };
    cpu.run();
}
