#[derive(Debug)]
struct RuleContainer {
    rule: Rule,
    address: usize,
}

#[derive(Debug)]
enum Rule {
    Char {
        character: char,
    },
    Ref {
        ops: Vec<Vec<usize>>,
    },
}

impl RuleContainer {
    fn matches<'rule, 'b, 'ruleset>(
        &'rule self,
        input: &'b str,
        ruleset: &'ruleset RuleSet,
    ) -> Option<&'b str> {
        match &self.rule {
            Rule::Char { character: c } => {
                if input.starts_with(*c) {
                    let (_, rest) = input.split_at(1);
                    Some(rest)
                } else {
                    None
                }
            }
            Rule::Ref { ops: options } => {
                options.iter().find_map(|option| {
                    option.iter().try_fold(input, |current_input, address| {
                        ruleset
                            .rules
                            .get(address)
                            .expect("couldn't find an address")
                            .matches(current_input, ruleset)
                    })
                })
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

type Rules = std::collections::HashMap<usize, RuleContainer>;
struct RuleSet {
    rules: Rules,
}

impl std::str::FromStr for RuleSet {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rules: Rules = std::collections::HashMap::new();
        for line in input.lines() {
            let rule: RuleContainer = line.parse()?;
            rules.insert(rule.address, rule);
        }

        Ok(Self { rules })
    }
}

impl RuleSet {
    fn is_match(&self, rule: &usize, input: &str) -> bool {
        if let Some(rule) = self.rules.get(rule) {
            let result = rule.matches(input, &self);
            matches!(result, Some(""))
        } else {
            false
        }
    }
}

impl std::error::Error for Error {}

impl std::str::FromStr for RuleContainer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().splitn(2, ": ");
        let pos = parts
            .next()
            .ok_or(Error::ParseError("didn't have position".to_string()))?
            .parse::<usize>()
            .map_err(|_| Error::ParseError(format!("couldn't parse line: {}", s)))?;
        let rule = parts
            .next()
            .ok_or(Error::ParseError("didn't have rule".to_string()))?;
        match rule.trim() {
            c if c.chars().any(char::is_alphabetic) => {
                let c: char = c
                    .chars()
                    .find(|c| c.is_alphabetic())
                    .ok_or(Error::ParseError("should be have alphabetic".to_string()))?;
                Ok(RuleContainer { rule: Rule::Char { character: c}, address: pos})
            }
            rules => {
                let refs = rules
                    .split(" | ")
                    .map(|rule| {
                        rule.split(" ")
                            .map(|num| {
                                num.parse::<usize>()
                                    .map_err(|e| Error::ParseError(format!("{}", e)))
                            })
                            .collect::<Result<Vec<usize>, Error>>()
                    })
                    .collect::<Result<_, Error>>()?;

                Ok(RuleContainer { rule: Rule::Ref { ops: refs}, address: pos })
            }
        }
    }
}

fn part1(s: &str) -> Result<usize, Error> {
    let mut parts = s.split("\n\n");
    let rule_str = parts.next().expect("couldn't get rules");
    let candidates_str = parts.next().expect("couldn't get candidates");
    let ruleset: RuleSet = rule_str.parse()?;

    Ok(candidates_str
        .lines()
        .map(|line| line.trim())
        .filter(|candidate| ruleset.is_match(&0, candidate))
        .count())
}

fn part2(s: &str) -> Result<usize, Error> {
    let mut parts = s.split("\n\n");
    let rule_str = parts.next().expect("couldn't get rules");
    let candidates_str = parts.next().expect("couldn't get candidates");
    let ruleset: RuleSet = rule_str.parse()?;

    let lines = candidates_str
        .lines()
        .map(|line| line.trim())
        .filter(|candidate| ruleset.is_match(&0, candidate))
        .collect::<Vec<_>>();

    for line in &lines {
        println!("{}", line)
    }

    Ok(lines.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("part1: {}", part1(&input)?);

    let input2 = std::fs::read_to_string("input2.txt")?;
    println!("part2: {}", part2(&input2)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        let input = r#"0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"

            ababbb
            bababa
            abbbab
            aaabbb
            aaaabbb"#;

        assert_eq!(part1(&input).unwrap(), 2);
    }

    #[test]
    fn test_case_ruleset() {
        let rule_str = r#"0: 8 11
            10: 23 14 | 28 1
            11: 42 31 | 42 11 31
            12: 24 14 | 19 1
            13: 14 3 | 1 12
            14: "b"
            15: 1 | 14
            16: 15 1 | 14 14
            17: 14 2 | 1 7
            18: 15 15
            19: 14 1 | 14 14
            1: "a"
            20: 14 14 | 1 15
            21: 14 1 | 1 14
            22: 14 14
            23: 25 1 | 22 14
            24: 14 1
            25: 1 1 | 1 14
            26: 14 22 | 1 20
            27: 1 6 | 14 18
            28: 16 1
            2: 1 24 | 14 4
            31: 14 17 | 1 13
            3: 5 14 | 16 1
            42: 9 14 | 10 1
            4: 1 1
            5: 1 14 | 15 1
            6: 14 14 | 1 14
            7: 14 5 | 1 21
            8: 42 | 42 8
            9: 14 27 | 1 26"#;

        let ruleset: RuleSet = rule_str.parse().unwrap();

        let test_cases = r#"bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        // for case in test_cases.lines() {
        //     let case = case.trim();
        //     assert!(ruleset.is_match(&0, case), "didn't match {}", case);
        // }
        //

        assert!(ruleset.is_match(&0, "babbbbaabbbbbabbbbbbaabaaabaaa"));
            

        // assert_eq!(ruleset.is_match(&0, "aaaabbaaaabbaaa"), false);
    }

    #[test]
    fn ruleset_works_without_loops() {
        let input = r#"42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        assert_eq!(part2(&input).unwrap(), 3);
    }
}
