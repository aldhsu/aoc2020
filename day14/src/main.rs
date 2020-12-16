use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
struct Computer<T: Masker + Debug> {
    mask: T,
    registers: HashMap<u64, u64>,
}

impl<T: Masker + Debug> Computer<T> {
    fn sum(&self) -> u64 {
        self.registers.values().sum()
    }
}

#[derive(Debug)]
struct Mask {
    ones: u64,
    zeroes: u64,
}

trait Masker {
    fn apply(&self, num: u64) -> Vec<u64>;
}

impl std::str::FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ones = s.chars().fold(0, |mut memo, c| {
            memo <<= 1;

            if c == '1' {
                memo |= 1;
            }
            memo
        });

        let zeroes = s.chars().fold(0, |mut memo, c| {
            memo <<= 1;

            if c != '0' {
                memo |= 1;
            }
            memo
        });

        Ok(Self { ones, zeroes })
    }
}

impl Masker for Mask {
    fn apply(&self, mut num: u64) -> Vec<u64> {
        num |= self.ones;
        num &= self.zeroes;
        vec![num]
    }
}

#[derive(Debug)]
struct Mask2 {
    flippables: Vec<usize>,
    ones: u64,
}

impl std::str::FromStr for Mask2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ones = s.chars().fold(0, |mut memo, c| {
            memo <<= 1;

            if c == '1' {
                memo |= 1;
            }
            memo
        });

        let flippables = s
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(i, c)| if c == 'X' { Some(i) } else { None })
            .collect();

        Ok(Self {
            ones,
            flippables,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BitFlag {
    On(usize),
    Off(usize),
}

fn build_combinations(positions: &[usize]) -> Vec<Vec<BitFlag>> {
    if let Some((first_pos, positions)) = positions.split_first() {
        let mut result: Vec<Vec<BitFlag>> = vec![
            vec![BitFlag::On(*first_pos)],
            vec![BitFlag::Off(*first_pos)],
        ];
        for position in positions {
            let mut temp = vec![];
            std::mem::swap(&mut result, &mut temp);

            for flags in temp.into_iter() {
                for &flag in [BitFlag::On(*position), BitFlag::Off(*position)].into_iter() {
                    let mut bit_flags = flags.clone();
                    bit_flags.push(flag);
                    result.push(bit_flags);
                }
            }
        }

        result
    } else {
        vec![]
    }
}

impl Masker for Mask2 {
    fn apply(&self, mut num: u64) -> Vec<u64> {
        num |= self.ones;
        let combinations = build_combinations(&self.flippables[..]);

        combinations
            .iter()
            .map(|flags| {
                let result = flags.iter().fold(num.clone(), |mut memo, flag| {
                    match flag {
                        BitFlag::On(position) => {
                            memo |= 1 << position;
                        }
                        BitFlag::Off(position) => {
                            memo &= !(1 << position);
                        }
                    }
                    memo
                });
                result
            })
            .collect()
    }
}

enum Op<T: Masker> {
    Mask(T),
    Set((u64, u64)),
}

impl Op<Mask> {
    fn apply(self, computer: &mut Computer<Mask>) {
        match self {
            Op::Mask(val) => computer.mask = val,
            Op::Set((address, value)) => {
                *computer.registers.entry(address).or_insert(0) =
                    computer.mask.apply(value).first().cloned().unwrap();
            }
        }
    }
}

impl Op<Mask2> {
    fn apply(self, computer: &mut Computer<Mask2>) {
        match self {
            Op::Mask(val) => computer.mask = val,
            Op::Set((address, value)) => {
                for combo in computer.mask.apply(address) {
                    *computer.registers.entry(combo).or_insert(0) = value
                }
            }
        }
    }
}

#[derive(Debug)]
enum Error {
    ParseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::error::Error for Error {}

impl<T: Debug + Masker + FromStr> FromStr for Op<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, " = ");
        let raw_op = parts
            .next()
            .ok_or(Error::ParseError("not enougn parts".to_string()))?
            .trim();
        if raw_op.starts_with("mask") {
            Ok(Op::Mask(
                parts
                    .next()
                    .ok_or(Error::ParseError("not enough parts".to_string()))?
                    .parse()
                    .map_err(|_| Error::ParseError("cannot parse into mask".to_string()))?,
            ))
        } else if raw_op.starts_with("mem") {
            let address = raw_op
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .map_err(|_| Error::ParseError("couldn't get address".to_string()))?;
            Ok(Op::Set((
                address,
                parts
                    .next()
                    .ok_or(Error::ParseError("not enough parts".to_string()))?
                    .parse::<u64>()
                    .map_err(|_| Error::ParseError("couldn't parse set value".to_string()))?,
            )))
        } else {
            return Err(Error::ParseError(format!("unknown op: {}", raw_op)));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut computer = Computer {
        mask: Mask { ones: 0, zeroes: 0 },
        registers: HashMap::new(),
    };
    for line in input.lines().map(|line| line.parse::<Op<Mask>>()) {
        let op = line?;
        op.apply(&mut computer);
    }
    let part1 = computer.sum();
    println!("part1: {}", part1);

    let mut computer = Computer {
        mask: Mask2 { flippables: vec![], ones: 0  },
        registers: HashMap::new(),
    };

    for line in input.lines().map(|line| line.parse::<Op<Mask2>>()) {
        let op = line?;
        op.apply(&mut computer);
    }

    let part2 = computer.sum();
    println!("part2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_apply() {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        let mut computer = Computer {
            mask: Mask { ones: 0, zeroes: 0 },
            registers: HashMap::new(),
        };
        for line in input.lines().map(|line| line.parse::<Op<Mask>>()) {
            let op = line.unwrap();
            op.apply(&mut computer);
        }

        assert_eq!(computer.sum(), 165);
    }

    #[test]

    fn it_can_apply2() {
        let input = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        let mut computer = Computer {
            mask: Mask2 {
                flippables: vec![],
                ones: 0,
            },
            registers: HashMap::new(),
        };
        for line in input.lines().map(|line| line.parse::<Op<Mask2>>()) {
            let op = line.unwrap();
            op.apply(&mut computer);
        }

        dbg!(&computer.registers);
        assert_eq!(computer.sum(), 208);
    }

    #[test]
    fn it_can_build_comboes2() {
        let vec = vec![1_usize, 2_usize];
        let result = build_combinations(&vec);

        assert_eq!(
            result,
            vec![
                vec![BitFlag::On(1), BitFlag::On(2)],
                vec![BitFlag::On(1), BitFlag::Off(2)],
                vec![BitFlag::Off(1), BitFlag::On(2)],
                vec![BitFlag::Off(1), BitFlag::Off(2)],
            ]
        );
    }
}
