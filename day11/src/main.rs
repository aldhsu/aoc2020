use std::convert::TryFrom;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
struct Map {
    inner: Vec<Tile>,
    width: usize,
    height: usize,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .inner
            .chunks(self.width)
            .map(|line| line.iter().map(|tile| char::from(tile)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "width: {}, height: {}\n{}",
            self.width, self.height, string
        )
    }
}

impl Map {
    const POSITIONS: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn next2(&mut self) -> bool {
        let mut changes = vec![];
        for i in 0..self.inner.len() {
            let pos = self.from_index(i);
            match &self.inner[i] {
                Tile::EmptySeat => {
                    if self.los_count(pos.0, pos.1) == 0 {
                        changes.push(i);
                    }
                }
                Tile::OccupiedSeat => {
                    if pos.0 == self.width as isize - 1 {}
                    if self.los_count(pos.0, pos.1) >= 5 {
                        changes.push(i);
                    }
                }
                _ => continue,
            }
        }

        changes.iter().for_each(|&i| match self.inner.get_mut(i) {
            Some(tile @ Tile::EmptySeat) => *tile = Tile::OccupiedSeat,
            Some(tile @ Tile::OccupiedSeat) => *tile = Tile::EmptySeat,
            _ => unreachable!(),
        });

        !changes.is_empty()
    }

    fn next(&mut self) -> bool {
        let mut changes = vec![];
        for i in 0..self.inner.len() {
            let pos = self.from_index(i);
            match &self.inner[i] {
                Tile::EmptySeat => {
                    if self.adjacent_count(pos.0, pos.1) == 0 {
                        changes.push(i);
                    }
                }
                Tile::OccupiedSeat => {
                    if pos.0 == self.width as isize - 1 {}
                    if self.adjacent_count(pos.0, pos.1) >= 4 {
                        changes.push(i);
                    }
                }
                _ => continue,
            }
        }

        changes.iter().for_each(|&i| match self.inner.get_mut(i) {
            Some(tile @ Tile::EmptySeat) => *tile = Tile::OccupiedSeat,
            Some(tile @ Tile::OccupiedSeat) => *tile = Tile::EmptySeat,
            _ => unreachable!(),
        });

        !changes.is_empty()
    }

    fn adjacent_count(&self, x: isize, y: isize) -> u32 {
        Self::POSITIONS
            .iter()
            .map(|(o_x, o_y)| {
                let pos = (x + o_x, y + o_y);
                if let Some(Tile::OccupiedSeat) = self.read(x + o_x, y + o_y) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn los_count(&self, x: isize, y: isize) -> u32 {
        Self::POSITIONS
            .iter()
            .map(|(o_x, o_y)| {
                let mut offset_x = *o_x;
                let mut offset_y = *o_y;
                loop {
                    match self.read(x + offset_x, y + offset_y) {
                        Some(Tile::EmptySeat) => return 0,
                        Some(Tile::OccupiedSeat) => return 1,
                        Some(Tile::Floor) => {
                            offset_x += o_x;
                            offset_y += o_y;
                        }
                        None => return 0,
                    }
                }
            })
            .sum()
    }

    fn read(&self, x: isize, y: isize) -> Option<&Tile> {
        if x.is_negative() || y.is_negative() {
            return None;
        }
        if x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        self.inner.get(self.width * y as usize + x as usize)
    }

    fn from_index(&self, index: usize) -> (isize, isize) {
        let y = index / self.width;
        let x = index % self.width;
        (x as isize, y as isize)
    }

    fn count_occupied_seats(&self) -> usize {
        self.inner
            .iter()
            .filter_map(|tile| {
                if let Tile::OccupiedSeat = tile {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::convert::From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        use Tile::*;

        match tile {
            EmptySeat => 'L',
            OccupiedSeat => '#',
            Floor => '.',
        }
    }
}

impl std::convert::TryFrom<&char> for Tile {
    type Error = ParseMapError;

    fn try_from(s: &char) -> Result<Self, Self::Error> {
        use Tile::*;

        Ok(match s {
            'L' => EmptySeat,
            '#' => OccupiedSeat,
            '.' => Floor,
            s => return Err(ParseMapError::UnknownChar(s.to_string())),
        })
    }
}

#[derive(Debug)]
enum ParseMapError {
    UnknownChar(String),
}
impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Tile> = s
            .lines()
            .flat_map(|line| line.trim().chars().map(|c| Tile::try_from(&c)))
            .collect::<Result<_, _>>()?;
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();

        Ok(Self {
            inner: map,
            width,
            height,
        })
    }
}

impl std::fmt::Display for ParseMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ParseMapError {}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut map: Map = input.parse()?;
    while map.next() {}

    let part1 = map.count_occupied_seats();
    println!("part1: {}", part1);

    let mut map: Map = input.parse()?;
    while map.next2() {
        println!("{}", map.count_occupied_seats());
    }

    let part2 = map.count_occupied_seats();
    println!("part2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL"#;
        let mut map: Map = input.parse().unwrap();
        let round1: Map = r#"#.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##"#
            .parse()
            .unwrap();

        map.next();
        assert_eq!(map, round1);

        let round2: Map = r#"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"#
            .parse()
            .unwrap();

        map.next();
        assert_eq!(map, round2);
    }

    #[test]
    fn it_can_get_index() {
        let round1: Map = r#"#.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##"#
            .parse()
            .unwrap();
        assert_eq!(round1.adjacent_count(9, 0), 3);
    }

    #[test]
    fn it_works_part2() {
        let input = r#"L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL"#;
        let mut map: Map = input.parse().unwrap();
        map.next2();
        let round1: Map = r#"#.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##"#
            .parse()
            .unwrap();
        assert_eq!(map, round1);

        map.next2();
        let round2: Map = r#"#.LL.LL.L#
            #LLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLLL.L
            #.LLLLL.L#"#
            .parse()
            .unwrap();
        assert_eq!(map, round2);
    }

    #[test]
    fn it_does_los_counts() {
        let map: Map = r#"#.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##"#
            .parse()
            .unwrap();
        assert_eq!(map.los_count(6, 0), 5);
    }
}
