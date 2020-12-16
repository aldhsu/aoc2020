use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

impl Rule {
    fn is_valid(&self, num: &u32) -> bool {
        for range in &self.ranges {
            if range.contains(num) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
enum Error {
    RuleParseError(&'static str),
    RuleCantParse(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl std::str::FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ": ");
        let name = parts
            .next()
            .ok_or(Error::RuleParseError("couldn't parse name"))?;
        let ranges = parts
            .next()
            .ok_or(Error::RuleParseError("couldn't read ranges"))?
            .splitn(2, " or ")
            .map(|range| {
                let mut nums = range.splitn(2, "-").map(|num| num.trim().parse::<u32>());

                let start = nums
                    .next()
                    .ok_or(Error::RuleParseError("no start of range"))?
                    .map_err(|_| Error::RuleParseError("couldn't parse start"))?;
                let end = nums
                    .next()
                    .ok_or(Error::RuleParseError("couldn't find end part"))?
                    .map_err(|_| Error::RuleCantParse(format!("couldn't parse end {:?}", range)))?;

                Ok(start..=end)
            })
            .collect::<Result<Vec<RangeInclusive<u32>>, Error>>()?;

        Ok(Self {
            name: name.to_string(),
            ranges,
        })
    }
}

fn part1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let rules = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|s| s.parse())
        .collect::<Result<Vec<Rule>, Error>>()?;
    let nearby_tickets = input
        .lines()
        .skip_while(|line| !line.starts_with("nearby tickets"))
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;

        let result = nearby_tickets
            .iter()
            .filter_map(|ticket| {
                for num in ticket {
                    if rules.iter().any(|rule| rule.is_valid(num)) {
                        continue
                    } else {
                        return Some(num)
                    }
                }
                None
            });
    Ok(result.sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let part1 = part1(&input);
    println!("part1: {}", part1?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;
        assert_eq!(part1(&input).unwrap(), 71);
    }
}
