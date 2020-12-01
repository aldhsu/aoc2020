use itertools::Itertools;

const TOTAL: i32 = 2020;

fn part2(nums: &[i32]) {
    let result = nums
        .iter()
        .combinations(3)
        .find(|items| items.iter().cloned().sum::<i32>() == TOTAL)
        .expect("couldn't find 3 values that total 2020");
    println!("{}", result.iter().fold(1, |memo, &&i| memo * i));
}

fn part1(nums: &[i32]) -> std::collections::HashSet<&i32> {
    let mut result = (None, None);

    let mut set = std::collections::HashSet::new();
    for num in nums {
        let complement = TOTAL - num;
        match set.get(&complement) {
            Some(_) => {
                result = (Some(num), Some(complement));
            }
            None => {
                set.insert(num);
            }
        };
    }

    match result {
        (Some(a), Some(b)) => println!("{}", a * b),
        _ => println!("couldn't find result"),
    }
    set
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("{}", input);
    let nums: Vec<_> = input
        .split("\n")
        .filter_map(|entry| entry.parse::<i32>().ok())
        .collect();

    part1(&nums);
    part2(&nums);
    Ok(())
}
