fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = std::fs::read_to_string("input.txt")?;
    let mut nums : Vec<i64>= input.lines().map(|line| line.parse::<i64>()).collect::<Result<_, _>>()?;
    nums.push(0);
    nums.sort();
    let max = nums.last().unwrap().clone();
    nums.push(max + 3);
    dbg!(&nums);
    let count = nums.windows(2).fold(std::collections::HashMap::new(), |mut memo, window| {
        let mut nums = window.iter();
        let first = nums.next().unwrap();
        let second = nums.next().unwrap();
        let diff = second - first;
        dbg!(diff);
        *memo.entry(diff).or_insert(0) += 1;
        memo
    });

    let mut map = std::collections::HashMap::new();
    for num in nums {
        let mut total : i64 = {
            [num - 1, num - 2, num - 3].iter().map(|num| {
                map.get(num).or(Some(&0)).unwrap()
            }).sum::<i64>()
        };
        if total == 0 {
            total = 1;
        }
        *map.entry(num).or_insert(0) += total;
    }

    dbg!(&map);

    println!("part1: {}", count.get(&1).unwrap() * count.get(&3).unwrap());
    println!("part1: {}", map.get(&max).unwrap());
    Ok(())
}
