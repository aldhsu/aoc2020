struct Recent(Option<usize>, Option<usize>);

enum RecentLen {
    Empty,
    One,
    Two,
}

impl Recent {
    fn new() -> Self {
        Self(None, None)
    }

    fn push(&mut self, val: usize) {
        self.1 = self.0;
        self.0 = Some(val);
    }

    fn len(&self) -> RecentLen {
        match (self.0, self.1) {
            (Some(_), Some(_)) => RecentLen::Two,
            (Some(_), None) => RecentLen::One,
            (None, None) => RecentLen::Empty,
            _ => unreachable!(),
        }
    }

    fn diff(&self) -> usize {
        match (self.0, self.1) {
            (Some(recent), Some(previous)) => recent - previous,
            _ => unreachable!(),
        }
    }
}

fn play_game(seed: &str, turn: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let seed_list = seed
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let len = seed_list.len();
    let mut last = seed_list.last().unwrap_or(&0).clone();
    let mut hash: std::collections::HashMap<usize, Recent> = std::collections::HashMap::new();

    for (i, num) in seed_list.into_iter().enumerate() {
        hash.entry(num).or_insert(Recent::new()).push(i);
    }

    for i in len..turn {
        let val =  {
            if let Some(prev_vals) = hash.get(&last) {
                match prev_vals.len() {
                    RecentLen::Empty => 0,
                    RecentLen::One => 0,
                    RecentLen::Two => { prev_vals.diff() }
                }
            } else { 0 }
        };
        last = val;
        hash.entry(val).or_insert(Recent::new()).push(i);
    }
    Ok(last)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let seed = "0,13,16,17,1,10,6";

    let part1 = play_game(seed, 2020)?;
    println!("part1: {}", part1);

    let part1 = play_game(seed, 30_000_000)?;
    println!("part2: {}", part1);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn play_game_works_with_test_data() {
        assert_eq!(play_game("1,3,2", 2020).unwrap(), 1);
        assert_eq!(play_game("2,1,3", 2020).unwrap(), 10);
        assert_eq!(play_game("1,2,3", 2020).unwrap(), 27);
        assert_eq!(play_game("2,3,1", 2020).unwrap(), 78);
        assert_eq!(play_game("3,2,1", 2020).unwrap(), 438);
        assert_eq!(play_game("3,1,2", 2020).unwrap(), 1836);
    }

    #[test]
    fn play_game_works_with_test_data_big_number() {
        assert_eq!(play_game("0,3,6", 30_000_000).unwrap(), 175594);
        // assert_eq!(play_game("2,1,3", 2020).unwrap(), 10);
        // assert_eq!(play_game("1,2,3", 2020).unwrap(), 27);
        // assert_eq!(play_game("2,3,1", 2020).unwrap(), 78);
        // assert_eq!(play_game("3,2,1", 2020).unwrap(), 438);
        // assert_eq!(play_game("3,1,2", 2020).unwrap(), 1836);
    }
}
