fn inv_mod(x: i64, p: i64) -> i64 {
    //Fermat's little theorem for primes
    //https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

fn main() {
    let mut input = r#"1002394
13,x,x,41,x,x,x,37,x,x,x,x,x,419,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,421,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17"#.lines();
    let time = input
        .next()
        .expect("can't get time")
        .parse::<i32>()
        .expect("Can't parse time");
    let buses = input
        .next()
        .expect("couldn't get time")
        .split(",")
        .filter_map(|time| time.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let part1 = buses
        .iter()
        .min_by(|&x, &y| {
            let total_x = (time / x) * x + x;
            let total_y = (time / y) * y + y;
            (total_x).cmp(&total_y)
        })
        .expect("couldn't find min");
    println!("part1: {}", part1 * (((time / part1) * part1 + part1) - time));

    let mut input = r#"1002394
13,x,x,41,x,x,x,37,x,x,x,x,x,419,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,421,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17"#;

    println!("part2: {}", part2(&input).unwrap());
}

fn part2(input: &str) -> Option<i64> {
    let buses = input
        .lines()
        .skip(1)
        .next()
        .expect("couldn't get time")
        .split(",")
        .map(|time| time.trim().parse::<i64>().ok())
        .collect::<Vec<Option<i64>>>();

    let mut buses = buses
        .into_iter()
        .enumerate()
        .filter(|(i, bus)| bus.is_some())
        .map(|(i, bus)| (i as i64, bus.unwrap()))
        .collect::<Vec<_>>();
    let prod = buses.iter().cloned().map(|(_, modl)| modl).product();

    // Chinese remainder theorem
    // https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html
    // I don't get the -a though, without it is a later time subtracting prod from the answer gives
    // negative the right answer
    Some(
        buses
            .iter()
            .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
            .sum::<i64>()
            .rem_euclid(prod),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_find_modulo_inverse() {
        assert_eq!(inv_mod(3_i64, 13_i64), 9);
    }

    #[test]
    fn it_can_work_for_2() {
        let input = r#"asdf
7,13,x,x,59,x,31,19
            "#;
        assert_eq!(part2(&input), Some(1068781));
    }
}
