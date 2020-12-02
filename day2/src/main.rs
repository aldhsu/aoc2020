use std::convert::TryInto;

trait Policy<'a> {
    type FP: std::convert::TryFrom<&'a str, Error = Error>;
    fn new(first_part: Self::FP, second_part: char) -> Self;

    fn evaluate(&self, candidate: &str) -> bool;

    fn valid_count(input: &'a str) -> Result<usize, Error>
    where
        Self: Sized,
    {
        let mut count = 0;

        for line in input.lines() {
            let mut iter = line.splitn(3, " ");
            let range: Self::FP = iter.next().ok_or(Error::NoFirstPartOfPolicy)?.try_into()?;
            let letter = iter
                .next()
                .ok_or(Error::NoLetterSpecified)?
                .chars()
                .next()
                .ok_or(Error::NoLetterSpecified)?;

            let policy = Self::new(range, letter);

            if policy.evaluate(iter.next().ok_or(Error::NoTestCandidate)?) {
                count += 1;
            }
        }
        Ok(count)
    }
}

struct RangePolicy {
    range: Range,
    letter: char,
}

impl Policy<'_> for RangePolicy {
    fn evaluate(&self, candidate: &str) -> bool {
        let letter_count = candidate.chars().fold(0, |mut count, c| {
            if c == self.letter {
                count += 1;
            }
            count
        });
        self.range.range.contains(&letter_count)
    }

    type FP = Range;

    fn new(first_part: Self::FP, second_part: char) -> Self {
        Self {
            range: first_part,
            letter: second_part,
        }
    }
}

struct Range {
    range: std::ops::RangeInclusive<i32>,
}

impl std::convert::TryFrom<&str> for Range {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut digits = value.splitn(2, "-").map(|digits| digits.parse::<i32>());

        Ok(Range {
            range: digits
                .next()
                .ok_or_else(|| {
                    Error::RangeConversionError(format!("couldn't find low digits {}", value))
                })?
                .map_err(|err| {
                    Error::RangeConversionError(format!("couldn't parse digit {}", err))
                })?
                ..=digits
                    .next()
                    .ok_or_else(|| {
                        Error::RangeConversionError(format!("couldn't find high digits {}", value))
                    })?
                    .map_err(|err| {
                        Error::RangeConversionError(format!("couldn't parse digit {}", err))
                    })?,
        })
    }
}

#[derive(Debug)]
enum Error {
    RangeConversionError(String),
    NoFirstPartOfPolicy,
    NoLetterSpecified,
    NoTestCandidate,
    Unparseable,
    NoFirstPosition,
    NoSecondPosition,
}

use std::fmt;
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
}

fn part1(input: &str) -> Result<(), Error> {
    println!("part1: {}", RangePolicy::valid_count(input)?);
    Ok(())
}

struct PositionPolicy {
    positions: Positions,
    letter: char,
}

struct Positions {
    first_position: usize,
    second_position: usize,
}

impl std::convert::TryFrom<&str> for Positions {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut positions = value.splitn(2, "-").map(|pos| pos.parse::<usize>());

        Ok(Positions {
            first_position: positions
                .next()
                .ok_or(Error::NoFirstPosition)?
                .map_err(|_| Error::Unparseable)?
                - 1,
            second_position: positions
                .next()
                .ok_or(Error::NoSecondPosition)?
                .map_err(|_| Error::Unparseable)?
                - 1,
        })
    }
}

impl Policy<'_> for PositionPolicy {
    fn evaluate(&self, candidate: &str) -> bool {
        let first = candidate
            .chars()
            .nth(self.positions.first_position)
            .expect("no char at first position")
            == self.letter;

        let second = candidate
            .chars()
            .nth(self.positions.second_position)
            .expect("no char at second position")
            == self.letter;

        first ^ second
    }

    type FP = Positions;

    fn new(first_part: Self::FP, second_part: char) -> Self {
        Self {
            positions: first_part,
            letter: second_part,
        }
    }
}

fn part2(input: &str) -> Result<(), Error>{
    let count = PositionPolicy::valid_count(input)?;
    println!("part2: {}", count);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}
