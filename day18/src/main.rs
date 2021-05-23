struct Calculator {}

#[derive(Debug)]
enum Error<'a> {
    SumError(String),
    Unparseable(&'a str, String),
}

impl<'a> std::error::Error for Error<'a> {}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq)]
enum Ops<'a> {
    Mul,
    Add,
    Num(u64),
    Parens(&'a str),
}

impl<'a> Ops<'a> {
    fn compute(&self, left: u64, right: u64) -> Result<u64, Error<'a>> {
        Ok(match self {
            Ops::Mul => left * right,
            Ops::Add => left + right,
            _ => return Err(Error::SumError(format!("tried to use {:?} as op", self))),
        })
    }

    fn coerce(self) -> Result<u64, Error<'a>> {
        Ok(match self {
            Ops::Num(num) => num,
            Ops::Parens(input) => Calculator::sum(input)?,
            _ => return Err(Error::SumError(format!("tried to use {:?} as num", self))),
        })
    }

    fn coerce2(self) -> Result<u64, Error<'a>> {
        Ok(match self {
            Ops::Num(num) => num,
            Ops::Parens(input) => Calculator::sum2(input)?,
            _ => return Err(Error::SumError(format!("tried to use {:?} as num", self))),
        })
    }
}

impl Calculator {
    fn parse_head(input: &str) -> Result<(Ops, &str), Error> {
        match input {
            input if input.starts_with('(') => {
                let end_index = {
                    let mut left_paren_count = 0;
                    let mut val = 0;
                    for (index, c) in input.chars().enumerate() {
                        match c {
                            '(' => left_paren_count += 1,
                            ')' => left_paren_count -= 1,
                            _ => {}
                        }

                        if left_paren_count == 0 {
                            val = index;
                            break;
                        }
                    }
                    if val == 0 {
                        Err(Error::Unparseable(
                            input,
                            "Couldn't find corresponding right paren".into(),
                        ))
                    } else {
                        Ok(val)
                    }
                }?;

                Ok((Ops::Parens(&input[1..end_index]), &input[end_index + 1..]))
            }
            input if input.starts_with(|c: char| c.is_digit(10)) => Ok((
                Ops::Num(
                    input
                        .chars()
                        .next()
                        .expect("checked already")
                        .to_digit(10)
                        .expect("already checked") as u64,
                ),
                &input[1..],
            )),
            input if input.starts_with('*') => Ok((Ops::Mul, &input[1..])),
            input if input.starts_with('+') => Ok((Ops::Add, &input[1..])),
            input => Err(Error::Unparseable(input, "couldn't parse".into())),
        }
    }

    fn sum(input: &str) -> Result<u64, Error> {
        let mut nodes = Vec::new();
        let mut input = input;
        loop {
            let (op, next_input) = Self::parse_head(input)?;
            nodes.push(op);
            input = next_input;
            if next_input.is_empty() {
                break;
            }
        }
        nodes.reverse();
        let mut total = 0;

        let first_num = nodes
            .pop()
            .ok_or_else(|| Error::SumError("no first term".into()))?
            .coerce()?;

        total += Ops::Add.compute(total, first_num)?;

        while !nodes.is_empty() {
            let op = nodes.pop().ok_or_else(|| Error::SumError("no op".into()))?;

            let num = nodes
                .pop()
                .ok_or_else(|| Error::SumError("no second term".into()))?
                .coerce()?;

            total = op.compute(total, num)?;
        }

        Ok(total)
    }

    fn parse_full(input: &str) -> Result<Vec<Ops>, Error> {
        let mut nodes = Vec::new();
        let mut input = input;
        loop {
            let (op, next_input) = Self::parse_head(input)?;
            nodes.push(op);
            input = next_input;
            if next_input.is_empty() {
                break;
            }
        }
        nodes.reverse();
        Ok(nodes)
    }

    fn sum2(input: &str) -> Result<u64, Error> {
        let mut nodes = Self::parse_full(input)?;

        // collapse additions
        loop {
            match nodes.iter().position(|node| matches!(node, Ops::Add)) {
                None => break,
                Some(index) => {
                    let left_index = index - 1;
                    let left = nodes.remove(left_index);
                    nodes.remove(left_index); // mul
                    let right = nodes.remove(left_index);
                    let new = Ops::Add.compute(left.coerce2()?, right.coerce2()?)?;

                    nodes.insert(left_index, Ops::Num(new))
                }
            }
        }

        let mut total = 1;
        for node in nodes.iter() {
            match node {
                Ops::Num(num) => total *= num,
                Ops::Parens(substr) => total *= Self::sum2(substr)?,
                _ => continue,
            }
        }

        Ok(total)
    }
}

fn part1<'a>(input: &'a str) -> Result<u64, Box<dyn std::error::Error + 'a>> {
    let mut total = 0;
    for line in input.lines() {
        total += Calculator::sum(&line)?;
    }
    Ok(total)
}

fn part2<'a>(input: &'a str) -> Result<u64, Box<dyn std::error::Error + 'a>> {
    let mut total = 0;
    for line in input.lines() {
        total += Calculator::sum2(&line)?;
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.replace(' ', "");
    match part1(&input) {
        Err(error) => println!("{}", error),
        Ok(total) => println!("part1: {}", total),
    }

    match part2(&input) {
        Err(error) => println!("{}", error),
        Ok(total) => println!("part2: {}", total),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_parse_parens() {
        assert_eq!(
            Calculator::parse_head("(1)").unwrap(),
            (Ops::Parens("1"), "")
        )
    }

    #[test]
    fn it_can_parse_double_parens() {
        assert_eq!(
            Calculator::parse_head("((1))").unwrap(),
            (Ops::Parens("(1)"), "")
        )
    }

    #[test]
    fn it_can_parse_confusing_parens() {
        assert_eq!(
            Calculator::parse_head("(1 + (1)) + (1)").unwrap(),
            (Ops::Parens("1 + (1)"), " + (1)")
        )
    }

    #[test]
    fn it_can_parse_digits() {
        assert_eq!(
            Calculator::parse_head("1 + (1)").unwrap(),
            (Ops::Num(1), " + (1)")
        )
    }

    #[test]
    fn it_can_sum_a_line() {
        assert_eq!(Calculator::sum("1+1").unwrap(), 2);
        assert_eq!(Calculator::sum("1+(1)").unwrap(), 2);
        assert_eq!(Calculator::sum("1*(1)").unwrap(), 1);
        assert_eq!(Calculator::sum("1*(1*2)").unwrap(), 2);
        assert_eq!(Calculator::sum("2*(1+2)").unwrap(), 6);
    }

    #[test]
    fn it_can_sum_a_line_advanced() {
        assert_eq!(Calculator::sum2("1+(2*3)+(4*(5+6))").unwrap(), 51);
        assert_eq!(Calculator::sum2("2*3+(4*5)").unwrap(), 46);
    }
}
