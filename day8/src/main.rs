use std::str::FromStr;

enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
enum Error {
    CantParse(String),
    CouldntGetOps(usize, String),
    InfiniteLoop,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl std::error::Error for Error {}

impl FromStr for Op {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.splitn(2, " ");
        let op = parts
            .next()
            .ok_or(Error::CantParse(format!("can't find op {}", line)))?;
        let number = parts
            .next()
            .ok_or(Error::CantParse(format!("can't find number {}", line)))?
            .parse::<i32>()
            .map_err(|e| Error::CantParse(format!("can't parse number {}", e)))?;

        use Op::*;
        Ok(match op {
            "acc" => Acc(number),
            "jmp" => Jmp(number),
            "nop" => Nop(number),
            _ => return Err(Error::CantParse(format!("unknown operation: {}", line))),
        })
    }
}

impl Op {
    fn run(&self, ctx: &mut Ctx) {
        use Op::*;
        match self {
            Jmp(val) => ctx.register = (ctx.register as i32 + val) as usize,
            Acc(val) => {
                ctx.accumulator += val;
                ctx.register += 1;
            }
            Nop(_) => {
                ctx.register += 1;
            }
        }
    }
}

struct Computer {
    ops: Vec<Op>,
    ctx: Ctx,
}

struct Ctx {
    accumulator: i32,
    register: usize,
}

impl FromStr for Computer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<Op> = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            ops,
            ctx: Ctx {
                accumulator: 0,
                register: 0,
            },
        })
    }
}

impl Computer {
    fn step(&mut self) -> Result<(usize, i32), Error> {
        let op = &self.ops.get(self.ctx.register).ok_or(Error::CouldntGetOps(
            self.ctx.register,
            format!(
                "tried to get {} but len is only {}",
                self.ctx.register,
                self.ops.len()
            ),
        ))?;
        op.run(&mut self.ctx);
        Ok((self.ctx.register, self.ctx.accumulator))
    }

    fn run(&mut self) -> Result<i32, Error> {
        self.reset();
        let mut map: std::collections::HashSet<usize> = std::collections::HashSet::new();

        loop {
            match self.step() {
                Ok((register, _)) => {
                    if !map.insert(register) {
                        return Err(Error::InfiniteLoop);
                    };
                }
                Err(Error::CouldntGetOps(_, _)) => return Ok(self.ctx.accumulator),
                Err(error) => return Err(error),
            }
        }
    }

    fn reset(&mut self) {
        self.ctx.accumulator = 0;
        self.ctx.register = 0;
    }
}

fn part2(computer: &mut Computer) -> Option<i32> {
    for i in 0..computer.ops.len() {
        let mut tmp: Op;
        match computer.ops.get_mut(i) {
            Some(jump @ Op::Jmp(_)) => {
                let nop_val = if let Op::Jmp(val) = jump {
                    *val
                } else {
                    unreachable!()
                };

                tmp = Op::Nop(nop_val);
                std::mem::swap(jump, &mut tmp);
            }
            Some(nop @ Op::Nop(_)) => {
                let jump_val = if let Op::Nop(val) = nop {
                    *val
                } else {
                    unreachable!()
                };

                tmp = Op::Jmp(jump_val);
                std::mem::swap(nop, &mut tmp);
            }
            _ => continue,
        }

        match computer.run() {
            Ok(accumulator) => {
                return Some(accumulator)
            },
            _ => {
                let op = computer
                    .ops
                    .get_mut(i)
                    .expect(&format!("couldn't get access to ops {}", i)[..]);
                std::mem::swap(op, &mut tmp);
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut computer: Computer = input.parse()?;
    let mut map: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut last_accumulator = 0;

    loop {
        let (register, accumulator): (usize, i32) = computer.step()?;
        if !map.insert(register) {
            break;
        };
        last_accumulator = accumulator
    }
    println!("part1: {}", last_accumulator);
    println!(
        "part2: {}",
        part2(&mut computer).expect("couldn't find result for part 2")
    );
    Ok(())
}
