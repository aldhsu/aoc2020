use nom::{branch::alt, bytes::complete::tag, multi::many0, IResult};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    E,
    W,
    SW,
    SE,
    NE,
    NW,
}

#[derive(Debug)]
enum Error {
    ParseError,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinates(i32, i32);

impl std::ops::Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Coordinates {
    fn neighbours(&self) -> NeighbourIter {
        NeighbourIter {
            index: 0,
            coordinates: self.clone(),
        }
    }
}

struct NeighbourIter {
    index: usize,
    coordinates: Coordinates,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::E,
    Direction::W,
    Direction::SW,
    Direction::SE,
    Direction::NE,
    Direction::NW,
];

impl Iterator for NeighbourIter {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == ALL_DIRECTIONS.len() {
            None
        } else {
            let offset: Coordinates = (&ALL_DIRECTIONS[self.index]).into();
            self.index += 1;
            Some(self.coordinates + offset)
        }
    }
}

impl From<&Direction> for Coordinates {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::E => Self(1, 0),
            Direction::W => Self(-1, 0),
            Direction::SW => Self(0, 1),
            Direction::SE => Self(1, 1),
            Direction::NE => Self(0, -1),
            Direction::NW => Self(-1, -1),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        Ok(match s {
            "e" => E,
            "w" => W,
            "ne" => NE,
            "nw" => NW,
            "sw" => SW,
            "se" => SE,
            _ => return Err(Error::ParseError),
        })
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((
        tag("sw"),
        tag("se"),
        tag("ne"),
        tag("nw"),
        tag("e"),
        tag("w"),
    ))(input)?;
    Ok((input, direction.parse().expect("couldn't get stuff")))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many0(parse_direction)(input)
}

struct Map {
    black: std::collections::HashSet<Coordinates>,
}

impl Map {
    fn initialize(input: &str) -> Self {
        let mut black = std::collections::HashSet::new();
        for line in input.lines() {
            let (left, directions) = parse_directions(line).unwrap();
            let coordinates = directions
                .iter()
                .fold(Coordinates(0, 0), |memo, direction| {
                    let offset: Coordinates = direction.into();
                    memo + offset
                });
            if !black.insert(coordinates) {
                black.remove(&coordinates);
            }
        }
        Self { black }
    }

    fn count(&self) -> usize {
        self.black.len()
    }

    fn tick(&mut self) {
        let mut influence_map = std::collections::HashMap::new();

        for coord in &self.black {
            for neighbour in coord.neighbours() {
                *influence_map.entry(neighbour).or_insert(0) += 1;
            }
        }

        // kill
        let kill = &self
            .black
            .iter()
            .filter(|coord| match influence_map.get(coord) {
                Some(val) if val > &2 => true,
                None => true,
                _ => false,
            })
            .cloned()
            .collect::<Vec<_>>();

        //new
        let new = influence_map
            .iter()
            .filter_map(|(coord, count)| match count {
                2 => Some(coord),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        for killed in kill {
            self.black.remove(killed);
        }

        for new_life in new {
            self.black.insert(new_life);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut map = Map::initialize(&input);
    println!("part1: {}", map.count());
    for _ in 0..100 {
        map.tick();
    }
    println!("part2: {}", map.count());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction::*;
    #[test]
    fn it_can_parse() {
        let (consumed, directions) = parse_directions("sewwseswwe").unwrap();
        assert_eq!(directions, vec![SE, W, W, SE, SW, W, E]);
    }

    #[test]
    fn it_can_map() {
        let (_, directions) = parse_directions("nwwswee").unwrap();
        assert_eq!(
            directions.iter().fold((0, 0), |memo, direction| {
                let Coordinates(x, y) = direction.into();
                (memo.0 + x, memo.1 + y)
            }),
            (0, 0)
        );
    }

    #[test]
    fn directions_cancel() {
        fn add_coordinates(a: Direction, b: Direction) -> (i32, i32) {
            let a = Coordinates::from(&a);
            let b = Coordinates::from(&b);
            (a.0 + b.0, b.1 + a.1)
        }

        use Direction::*;
        assert_eq!(add_coordinates(W, E), (0, 0));
        assert_eq!(add_coordinates(NW, SE), (0, 0));
        assert_eq!(add_coordinates(SW, NE), (0, 0));
    }

    #[test]
    fn larger_sample() {
        let input = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;
        let mut map = Map::initialize(&input);
        assert_eq!(map.count(), 10);
        for _ in 0..100 {
            map.tick();
        }
        assert_eq!(map.count(), 2208);
    }
}
