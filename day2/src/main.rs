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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let count = input.lines().filter_map(|line| {
        let mut iter = line.split(" ");
        let range: Range = iter.next().expect("no range in string").into();
        let letter = iter
            .next()
            .expect("no letter specified")
            .chars()
            .next()
            .expect("rule char not found");
        let policy = Policy {
            range,
            letter,
        };

        if policy.evaluate(iter.next().expect("no test string found")) {
            Some(())
        } else {
            None
        }
    }).count();
    println!("part1: {}", count);
    Ok(())
}
