use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
struct Game {
    player1: VecDeque<u32>,
    player2: VecDeque<u32>,
}

impl Game {
    fn play_hand(&mut self) -> bool {
        let c1 = self.player1.pop_front().expect("no player1 cards");
        let c2 = self.player2.pop_front().expect("no player2 cards");
        let mut winner = match c1.cmp(&c2) {
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
    let mut game = Game { player1, player2 };
    while game.play_hand() {}
    dbg!(game.score());
    dbg!(game);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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

        let mut count = 0;
        while game.play_hand() {
            count += 1;
            dbg!(&count);
            dbg!(&game);
        }
        dbg!(&game);
        assert_eq!(game.score(), 306);

    }
}
