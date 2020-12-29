use itertools::Itertools;
use std::collections::BTreeSet;
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut parts = line.splitn(2, " (contains");
            let ingredients = parts
                .next()
                .expect("couldn't get ingredients")
                .split(" ")
                .map(str::to_string)
                .collect::<BTreeSet<String>>();
            let allergens = parts
                .next()
                .expect("couldn't get allergens")
                .split(")")
                .next()
                .expect("couldn't drop )")
                .split(", ")
                .map(|allergen| allergen.trim().to_string())
                .collect::<BTreeSet<String>>();
            ((i, allergens), ingredients)
        })
        .collect::<HashMap<_, _>>();

    dbg!(map.keys().count());

    let mut list_of_allergen_translations: HashMap<String, BTreeSet<String>> = HashMap::new();

    for allergens in map.keys() {
        for allergen in allergens.1.iter() {
            match list_of_allergen_translations.entry(allergen.clone()) {
                std::collections::hash_map::Entry::Occupied(val) => continue,
                std::collections::hash_map::Entry::Vacant(slot) => {
                    let mut iter = map.iter().filter_map(|(k, v)| {
                        if k.1.contains(allergen) {
                            Some(v)
                        } else {
                            None
                        }
                    });
                    let mut result = iter.next().expect("couldn't find allergen").clone();
                    result = iter.fold(result, |memo, translations| {
                        memo.intersection(translations).cloned().collect()
                    });
                    slot.insert(result);
                }
            }
        }
    }

    let all_translations = map.values().fold(BTreeSet::new(), |memo, trans| {
        memo.union(trans).cloned().collect()
    });
    let all_allergen_translations = list_of_allergen_translations
        .values()
        .fold(BTreeSet::new(), |memo, trans| {
            memo.union(trans).cloned().collect()
        });

    all_translations
        .difference(&all_allergen_translations)
        .fold(0, |count, not_allergen| {
            count
                + (map
                    .values()
                    .filter(|symbols| symbols.contains(not_allergen))
                    .count())
        })
}

fn part2(input: &str) {
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut parts = line.splitn(2, " (contains");
            let ingredients = parts
                .next()
                .expect("couldn't get ingredients")
                .split(" ")
                .map(str::to_string)
                .collect::<BTreeSet<String>>();
            let allergens = parts
                .next()
                .expect("couldn't get allergens")
                .split(")")
                .next()
                .expect("couldn't drop )")
                .split(", ")
                .map(|allergen| allergen.trim().to_string())
                .collect::<BTreeSet<String>>();
            ((i, allergens), ingredients)
        })
        .collect::<HashMap<_, _>>();

    dbg!(map.keys().count());

    let mut list_of_allergen_translations: HashMap<String, BTreeSet<String>> = HashMap::new();

    for allergens in map.keys() {
        for allergen in allergens.1.iter() {
            match list_of_allergen_translations.entry(allergen.clone()) {
                std::collections::hash_map::Entry::Occupied(val) => continue,
                std::collections::hash_map::Entry::Vacant(slot) => {
                    let mut iter = map.iter().filter_map(|(k, v)| {
                        if k.1.contains(allergen) {
                            Some(v)
                        } else {
                            None
                        }
                    });
                    let mut result = iter.next().expect("couldn't find allergen").clone();
                    result = iter.fold(result, |memo, translations| {
                        memo.intersection(translations).cloned().collect()
                    });
                    slot.insert(result);
                }
            }
        }
    }

    let mut sorted = list_of_allergen_translations
        .into_iter()
        .collect::<Vec<(String, BTreeSet<String>)>>();
    sorted.sort();
    let mut comboes = sorted
        .iter()
        .map(|(allergen, translations)| translations)
        .multi_cartesian_product();
    let uniq_combo = comboes
        .find(|combo| combo.iter().unique().count() == sorted.len())
        .expect("coudn't find a combo that works");

    println!(
        "part2: {}",
        uniq_combo
            .into_iter()
            .cloned()
            .intersperse(",".to_string())
            .collect::<String>()
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("part1: {}", part1(&input));
    part2(&input);
    // println!("part2: {}", );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;
        assert_eq!(part1(&input), 5);
    }
}
