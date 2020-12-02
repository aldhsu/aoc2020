struct Policy {
    range: Range,
    letter: char,
}

impl Policy {
    pub fn evaluate(&self, candidate: &str) -> bool {
        let letter_count = candidate.chars().fold(0, |mut count, c| {
            if c == self.letter {
                count += 1;
            }
            count
        });
        letter_count >= self.range.low && letter_count <= self.range.high
    }
}

struct Range {
    low: i32,
    high: i32,
}

impl Into<Range> for &str {
    fn into(self) -> Range {
        let mut digits = self
            .split("-")
            .map(|digits| digits.parse::<i32>().expect("couldn't convert digit"));
        Range {
            low: digits.next().expect("didn't have enough digits for low"),
            high: digits.next().expect("didn't have enough digits for high"),
        }
    }
}

fn part1(input: &str) {
    let count = input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split(" ");
            let range: Range = iter.next().expect("no range in string").into();
            let letter = iter
                .next()
                .expect("no letter specified")
                .chars()
                .next()
                .expect("rule char not found");
            let policy = Policy { range, letter };

            if policy.evaluate(iter.next().expect("no test string found")) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("part1: {}", count);
}

struct Policy2 {
    positions: Positions,
    letter: char,
}

struct Positions {
    first_position: usize,
    second_position: usize,
}

impl Into<Positions> for &str {
    fn into(self) -> Positions {
        let mut positions = self
            .split("-")
            .map(|pos| pos.parse::<usize>().expect("could not parse into position"));

        Positions {
            first_position: positions.next().expect("could not parse first position") - 1,
            second_position: positions.next().expect("could not parse second position") - 1,
        }
    }
}

impl Policy2 {
    fn evaluate(&self, candidate: &str) -> bool {
        let first = candidate
            .chars()
            .nth(self.positions.first_position)
            .expect("no char at first position") == self.letter;

        let second = candidate
            .chars()
            .nth(self.positions.second_position)
            .expect("no char at second position") == self.letter;

        first ^ second
    }
}

fn part2(input: &str) {
    let count = input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split(" ");
            let positions: Positions = iter.next().expect("no range in string").into();
            let letter = iter
                .next()
                .expect("no letter specified")
                .chars()
                .next()
                .expect("rule char not found");
            let policy = Policy2 { positions, letter };

            if policy.evaluate(iter.next().expect("no test string found")) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("part2: {}", count);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    part1(&input);
    part2(&input);
    Ok(())
}
