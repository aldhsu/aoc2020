type RulesMap = std::collections::HashMap<String, Vec<(String, usize)>>;

fn part1(rules: &RulesMap) -> usize {
    fn find_matching_inner<'a>(rules: &'a RulesMap, mut key: &'_ str) -> Vec<&'a str> {
        if key.ends_with('s') {
            key = &key[..key.len() - 1] // drop s;
        }
        rules.iter().filter_map(|(outer, inner)| {
            if inner.iter().any(|(color, _)| color.contains(key)) {
                Some(&outer[..])
            } else {
                None
            }
        }).collect()
    }
    let mut color_to_be_searched = vec!["shiny gold bag"];
    let mut results = vec![];
    let mut searched = std::collections::HashSet::new();
    while let Some(color) = color_to_be_searched.pop() {
        match searched.insert(color) {
            false => continue,
            true => {
                results.push(color);
                color_to_be_searched.append(&mut find_matching_inner(&rules, color));
            }
        };
    }

    results.len() - 1
}

use std::borrow::Cow;

fn part2(rules: &RulesMap) -> usize {
    fn count_matching_inner<'a>(rules: &'a RulesMap, mut key: &'_ str) -> usize {
        let mut plural_key = Cow::from(key);

        if !key.ends_with('s') {
            plural_key += "s";
        };
        let rule = rules.get(plural_key.as_ref()).expect(&format!("unknown key {}", key));
        rule.iter().map(|(key, num)| {
            count_matching_inner(&rules, key) * num + num
        }).sum()
    }

    count_matching_inner(&rules, "shiny gold bags")
}

fn create_rules_map(input: &str) -> RulesMap {
    input.lines().map(|line| {
        let line = &line[..(line.len() - 1)]; // drop the full stop
        let mut parts = line.splitn(2, "contain ");
        let container = parts.next().expect("coudln't find a container").trim();
        let inner_parts = parts.next().expect("coudln't find a inner").split(", ").filter_map(|inner| {
            if inner.contains("no other bags") {
                None
            } else {
                let mut rule = inner.splitn(2, " ");
                let number = rule.next().expect("couldn't find number for rule").parse().expect("couldn't parse rule number");
                let color = rule.next().expect("couldn't find color for rule").trim();
                Some((color.to_string(), number))
            }
        }).collect::<Vec<_>>();
        (container.to_string(), inner_parts)
    }).collect::<RulesMap>()
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = std::fs::read_to_string("input.txt")?;
    let rules = create_rules_map(&input);
    println!("part1: {}", part1(&rules));
    println!("part2: {}", part2(&rules));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_part1() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let rules = create_rules_map(&input);
        assert_eq!(part1(&rules), 4);
    }

    #[test]
    fn it_works_bright_white() {
        let input = "bright white bags contain 1 shiny gold bag.";
        let rules = create_rules_map(&input);

        assert_eq!(part1(&rules), 1);
    }

    #[test]
    fn it_works_part2() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let rules = create_rules_map(&input);
        assert_eq!(part2(&rules), 32);
    }
}
