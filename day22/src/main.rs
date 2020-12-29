use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug)]
struct Game {
    player1: VecDeque<u32>,
    player2: VecDeque<u32>,
}

impl Game {
    fn play_hand(&mut self) -> bool {
        let c1 = self.player1.pop_front().expect("no player1 cards");
        let c2 = self.player2.pop_front().expect("no player2 cards");
        let winner = match c1.cmp(&c2) {
            Ordering::Equal => unreachable!(),
            Ordering::Less => &mut self.player2,
            Ordering::Greater => &mut self.player1,
        };
        let mut items = [c1, c2];
        items.sort();

        winner.extend(items.iter().rev());

        self.player1.len() > 0 && self.player2.len() > 0
    }

    fn score(&self) -> u32 {
        [&self.player1, &self.player2]
            .iter()
            .filter(|p| p.len() > 0)
            .next()
            .expect("no winner")
            .iter()
            .rev()
            .enumerate()
            .fold(0, |memo, (i, item)| memo + item * (i as u32 + 1))
    }
}

#[derive(Debug)]
struct Game2 {
    player1: VecDeque<u32>,
    player2: VecDeque<u32>,
    seen: SeenMap,
}

type SeenMap = std::collections::HashSet<(VecDeque<u32>, VecDeque<u32>)>;

#[derive(Debug)]
enum Player {
    One,
    Two,
}

impl Game2 {
    fn play_game(&mut self) -> Player {
        while self.play_hand() {}
        match self.winner() {
            None => Player::One,
            Some(player) => player,
        }
    }

    fn determine_winner(
        p1: &mut VecDeque<u32>,
        p2: &mut VecDeque<u32>,
    ) -> (Player, (u32, u32)) {
        let c1 = p1.pop_front().expect("no player1 cards");
        let c2 = p2.pop_front().expect("no player2 cards");
        p1.make_contiguous();
        p2.make_contiguous();

        if c1 <= p1.len() as u32 && c2 <= p2.len() as u32 {
            let player1 =
                VecDeque::from_iter(&mut p1.as_slices().0[0..c1 as usize].iter().cloned());
            let player2 =
                VecDeque::from_iter(&mut p2.as_slices().0[0..c2 as usize].iter().cloned());
            let mut other_game = Game2 {
                player1,
                player2,
                seen: std::collections::HashSet::new(),
            };

            match other_game.play_game() {
                Player::One => return (Player::One, (c1, c2)),
                Player::Two => return (Player::Two, (c2, c1)),
            }
        } else {
            match c1.cmp(&c2) {
                Ordering::Equal => unreachable!(),
                Ordering::Greater => return (Player::One, (c1, c2)),
                Ordering::Less => return (Player::Two, (c2, c1)),
            }
        }
    }

    fn play_hand(&mut self) -> bool {
        if !self.seen.insert((self.player1.clone(), self.player2.clone())) {
            return false;
        }
        let (winner, (item1, item2)) =
            Self::determine_winner(&mut self.player1, &mut self.player2);
        let winner = match winner {
            Player::One => &mut self.player1,
            Player::Two => &mut self.player2,
        };

        winner.extend([item1, item2].iter());

        self.winner().is_none()
    }

    fn winner(&self) -> Option<Player> {
        Some(match (self.player1.len(), self.player2.len()) {
            (0, _) => Player::Two,
            (_, 0) => Player::One,
            _ => return None,
        })
    }

    fn score(&self) -> u32 {
        [&self.player1, &self.player2]
            .iter()
            .filter(|p| p.len() > 0)
            .next()
            .expect("no winner")
            .iter()
            .rev()
            .enumerate()
            .fold(0, |memo, (i, item)| memo + item * (i as u32 + 1))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player1 = r#"38
        39
        42
        17
        13
        37
        4
        10
        2
        34
        43
        41
        22
        24
        46
        19
        30
        50
        6
        44
        28
        27
        36
        5
        45"#
    .lines()
    .map(|num| num.trim().parse::<u32>())
    .collect::<Result<VecDeque<_>, _>>()?;

    let player2 = r#"31
        40
        25
        11
        3
        48
        16
        9
        33
        7
        12
        35
        49
        32
        26
        47
        14
        8
        20
        23
        1
        29
        15
        21
        18"#
    .lines()
    .map(|num| num.trim().parse::<u32>())
    .collect::<Result<VecDeque<_>, _>>()?;
    let mut game = Game {
        player1: player1.clone(),
        player2: player2.clone(),
    };
    while game.play_hand() {}

    println!("part1: {}", game.score());

    let mut game = Game2 {
        player1: player1.clone(),
        player2: player2.clone(),
        seen: SeenMap::new(),
    };
    while game.play_hand() {}

    println!("part2: {}", game.score());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn part1() {
        let player1 = r#"9
            2
            6
            3
            1"#
        .lines()
        .map(|num| num.trim().parse::<u32>())
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();

        let player2 = r#"5
            8
            4
            7
            10"#
        .lines()
        .map(|num| num.trim().parse::<u32>())
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();
        let mut game = Game { player1, player2 };

        while game.play_hand() {}
        assert_eq!(game.score(), 306);
    }

    // #[test]
    fn it_doesnt_infinitely_loop() {
        let player1 = r#"43
            19"#
        .lines()
        .map(|num| num.trim().parse::<u32>())
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();

        let player2 = r#"2
            29
            14"#
        .lines()
        .map(|num| num.trim().parse::<u32>())
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();

        let mut game = Game2 {
            player1,
            player2,
            seen: std::collections::HashSet::new(),
        };

        while game.play_hand() {}
        // it doesn't loop infinitely
    }

    #[test]
    fn you_can_play_a_game_of_recursive_combat() {
        let player1 = r#"9
            2
            6
            3
            1"#
        .lines()
        .map(|num| num.trim().parse::<u32>())
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();

        let player2 = r#"5
            8
            4
            7
            10"#
        .lines()
        .map(|num| num.trim().parse::<u32>())
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap();

        let mut game = Game2 {
            player1,
            player2,
            seen: std::collections::HashSet::new(),
        };

        game.play_hand();
        assert_eq!(game.player1, vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(game.player2, vec![8, 4, 7, 10]);

        game.play_hand();
        assert_eq!(game.player1, vec![6, 3, 1, 9, 5]);
        assert_eq!(game.player2, vec![4, 7, 10, 8, 2]);

        game.play_hand();
        assert_eq!(game.player1, vec![3, 1, 9, 5, 6, 4]);
        assert_eq!(game.player2, vec![7, 10, 8, 2]);

        game.play_hand();
        assert_eq!(game.player1, vec![1, 9, 5, 6, 4]);
        assert_eq!(game.player2, vec![10, 8, 2, 7, 3]);

        game.play_hand();
        assert_eq!(game.player1, vec![9, 5, 6, 4]);
        assert_eq!(game.player2, vec![8, 2, 7, 3, 10, 1]);

        game.play_hand();
        assert_eq!(game.player1, vec![5, 6, 4, 9, 8]);
        assert_eq!(game.player2, vec![2, 7, 3, 10, 1]);

        game.play_hand();
        assert_eq!(game.player1, vec![6, 4, 9, 8, 5, 2]);
        assert_eq!(game.player2, vec![7, 3, 10, 1]);

        game.play_hand();
        assert_eq!(game.player1, vec![4, 9, 8, 5, 2]);
        assert_eq!(game.player2, vec![3, 10, 1, 7, 6]);

        game.play_hand();
        assert_eq!(game.player1, vec![9, 8, 5, 2]);
        assert_eq!(game.player2, vec![10, 1, 7, 6, 3, 4]);

        while game.play_hand() {}
        assert_eq!(game.score(), 291);
    }
}
