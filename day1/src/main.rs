use itertools::Itertools;

const TOTAL: i32 = 2020;

fn find_total(total: &i32, nums: &[i32]) -> Option<(i32, i32)> {
    let mut map = std::collections::HashSet::new();

    for &num in nums.iter() {
        let complement = total - num;
        match map.get(&complement) {
            Some(complement_index) => return Some((num, complement)),
            None => map.insert(num),
        };
    }

    None
}

fn part2(nums: &[i32]) {
    let duos : Vec<(i32, Vec<&i32>)> = nums
        .iter()
        .combinations(2)
        .map(|nums| {
            (nums.iter().cloned().sum(), nums)
        }
        )
        .collect();

    let hashset = nums.iter().collect::<std::collections::HashSet<&i32>>();
    for (duo_total, nums) in duos {
        let complement = TOTAL - duo_total;
        match hashset.get(&complement) {
            Some(third) => {
                let duo_product = nums.iter().cloned().fold(1_i32, std::ops::Mul::mul);
                println!("part2: {}", third.clone() * duo_product);
                return;
            }
            None => {}
        }
    }
}

fn part1(nums: &[i32]) {
    let result = find_total(&TOTAL, nums);

    match result {
        Some((a, b)) => println!("part1: {}", a * b),
        _ => println!("couldn't find result"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let nums: Vec<_> = input
        .split("\n")
        .filter_map(|entry| entry.parse::<i32>().ok())
        .collect();

    let hashset = nums.iter().collect::<std::collections::HashSet<&i32>>();
    assert!(hashset.len() == 200);

    part1(&nums);
    part2(&nums);
    Ok(())
}
