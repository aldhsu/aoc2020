#[derive(Debug)]
struct Group {
    count: usize,
}

const A: u8 = 97;

fn take_group2<'a>(input: &'a str) -> Group {
    let count = input
        .lines()
        .fold(0b1111_1111_1111_1111_1111_1111_11 as u32, |mask, line| {
            mask & line
                .chars()
                .fold(0, |bit_mask, c| bit_mask | 1 << (c as u8 - A))
        })
        .count_ones();

    Group {
        count: count as usize,
    }
}

fn take_group<'a>(input: &'a str) -> Group {
    let count = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<std::collections::HashSet<char>>()
        .len();

    Group { count }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let groups = input.split("\n\n");

    let part1: usize = groups.map(|g| take_group(g).count).sum();
    println!("part1: {}", part1);

    let part2: usize = part2(&input);
    println!("part2: {}", part2);
    Ok(())
}

fn part2(input: &str) -> usize {
    let groups = input.split("\n\n");
    groups.map(|g| take_group2(g).count).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn it_works_for_abc() {
        let input = "abc";
        assert_eq!(part2(&input), 3);
    }
}
