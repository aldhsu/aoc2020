const TOTAL: i32 = 2020;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("{}", input);
    let nums: Vec<_> = input
        .split("\n")
        .filter_map(|entry| 
             entry.parse::<i32>().ok()
             )
        .collect();
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
    Ok(())
}
