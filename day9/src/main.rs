#[derive(Debug)]
enum Error {
    NoResult,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}
type Map = std::collections::HashMap<i64, Vec<usize>>;
fn part1(nums: &Vec<i64>, preamble_length: usize) -> Result<i64, Box<dyn std::error::Error>> {
    let mut previous25: std::collections::HashMap<i64, Vec<usize>> = nums
        .iter()
        .enumerate()
        .take(preamble_length)
        .fold(std::collections::HashMap::new(), |mut memo, (i, n)| {
            memo.entry(*n).or_insert(vec![]).push(i);
            memo
        });

    fn check_digit(map: &Map, num: &i64) -> bool {
        for key in map.keys() {
            let complement = (key - num).abs();
            if let Some(pos) = map.get(&complement) {
                if &complement == key {
                    if pos.len() > 1 {
                        return true;
                    } else {
                        continue;
                    };
                } else {
                    return true;
                }
            } else {
                continue;
            }
        }
        false
    }

    for i in preamble_length..nums.len() {
        let dropped = nums[i - preamble_length];
        let check = nums[i];

        if check_digit(&previous25, &check) {
            if previous25
                .get(&dropped)
                .expect("can't get num about to be dropped")
                .len()
                == 1
            {
                previous25.remove(&dropped);
            } else {
                previous25.get_mut(&dropped).unwrap().remove(0);
            }
            previous25.entry(check).or_insert(vec![]).push(i);
            continue;
        } else {
            return Ok(check);
        }
    }
    Err(Box::new(Error::NoResult))
}

fn part2(nums: &Vec<i64>, num: &i64) -> Result<i64, Box<dyn std::error::Error>> {
    for start in 0..nums.len() {
        let mut total = 0_i64;

        for next in start..nums.len() {
            total += nums[next];
            if &total == num {
                return Ok(nums[start..=next].iter().max().unwrap()
                    + nums[start..=next].iter().min().unwrap());
            } else if &total > num {
                break;
            }
        }
    }

    Err(Box::new(Error::NoResult))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let nums = input
        .lines()
        .map(|num| num.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;

    let part1 = part1(&nums, 25)?;
    println!("part1: {}", part1);
    println!("part2: {}", part2(&nums, &part1)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        assert_eq!(part1(&input, 5).unwrap(), 127_i64);
    }

    #[test]
    fn it_works_part2() {
        let input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        assert_eq!(part2(&input, &127).unwrap(), 62_i64);
    }
}
