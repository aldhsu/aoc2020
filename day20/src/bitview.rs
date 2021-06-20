use crate::coordinate::Coordinate;
use crate::direction::Direction;
use crate::error::Error;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BitView {
    pub bits: Vec<bool>,
    pub side_len: usize,
}

impl BitView {
    pub fn without_sides(&self) -> BitView {
        let bits = (0..self.side_len)
            .flat_map(|y| {
                (0..self.side_len).filter_map(move |x| match (x, y) {
                    (0, _) => None,
                    (_, 0) => None,
                    (9, _) => None,
                    (_, 9) => None,
                    (x, y) => Some(*self.get(&(x, y)).unwrap()),
                })
            })
            .collect::<Vec<bool>>();

        BitView {
            bits,
            side_len: self.side_len - 2,
        }
    }

    pub fn get(&self, (x, y): &Coordinate) -> Option<&bool> {
        self.bits.get(y * self.side_len + x)
    }

    pub fn get_side(&self, dir: &Direction) -> impl Iterator<Item = Option<&bool>> + '_ {
        dir.coordinates_for_side(10)
            .map(move |coord| self.get(&coord))
    }

    fn set(&mut self, (x, y): Coordinate, value: bool) {
        self.bits[y * self.side_len + x] = value;
    }

    pub fn rotate(&self) -> Self {
        let mut clone = self.clone();

        for i in 0..=(self.side_len / 2 - 1) {
            for j in i..=(self.side_len - i - 2) {
                let tmp = self.get(&(i, j));

                clone.set((i, j), *self.get(&(self.side_len - j - 1, i)).unwrap());

                clone.set(
                    (self.side_len - j - 1, i),
                    *self
                        .get(&(self.side_len - i - 1, self.side_len - j - 1))
                        .unwrap(),
                );

                clone.set(
                    (self.side_len - i - 1, self.side_len - j - 1),
                    *self.get(&(j, self.side_len - i - 1)).unwrap(),
                );

                clone.set((j, self.side_len - i - 1), *tmp.unwrap());
            }
        }

        clone
    }

    pub fn flip(&self) -> Self {
        let mut clone = self.bits.clone();
        for x in 0..(self.side_len / 2) {
            for y in 0..(self.side_len) {
                let complement = self.side_len - x - 1;
                clone.swap(y * self.side_len + x, y * self.side_len + complement);
            }
        }

        Self {
            bits: clone,
            side_len: self.side_len,
        }
    }

    pub fn all_complements(self) -> Vec<Self> {
        let mut all = Vec::with_capacity(8);
        let mut current = self;
        for _ in 0..4 {
            let next = current.rotate();
            all.push(current);
            current = next;
        }
        current = current.rotate().flip();
        for _ in 0..4 {
            let next = current.rotate();
            all.push(current);
            current = next;
        }
        assert!(all.len() == 8);
        all
    }
}

impl std::str::FromStr for BitView {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: Vec<bool> = s
            .lines()
            .flat_map(|line| {
                line.trim().chars().map(|c| {
                    Ok(match c {
                        '#' => true,
                        '.' => false,
                        _ => return Err(Error::CantMatchChar(c)),
                    })
                })
            })
            .collect::<Result<_, Error>>()?;

        let side_len = (bits.len() as f32).sqrt() as usize;
        if bits.len() != side_len * side_len {
            return Err(Error::WrongNumber(format!("got {}", bits.len())));
        }

        Ok(Self { bits, side_len })
    }
}

impl std::fmt::Display for BitView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .bits
            .chunks(self.side_len)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| if *c { '#' } else { '.' })
                    .collect::<String>()
            })
            .intersperse("\n".to_string())
            .collect::<String>();
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitview_can_rotate() {
        let input = r#"##########
            ##########
            ##########
            ##########
            ##########
            ..........
            ..........
            ..........
            ..........
            .........."#;
        let mut image = input.parse::<BitView>().unwrap();

        let expected = r#"#####.....
#####.....
#####.....
#####.....
#####.....
#####.....
#####.....
#####.....
#####.....
#####....."#;
        image = image.rotate();
        assert_eq!(image.to_string(), expected);

        let expected = r#"..........
..........
..........
..........
..........
##########
##########
##########
##########
##########"#;
        image = image.rotate();
        assert_eq!(image.to_string(), expected);
    }

    #[test]
    fn it_can_flip() {
        let input = r#"
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####....."#;
        let image = input.parse::<BitView>().unwrap();

        let expected = r#".....#####
.....#####
.....#####
.....#####
.....#####
.....#####
.....#####
.....#####
.....#####
.....#####"#;
        assert_eq!(expected, image.flip().to_string());
    }

    #[test]
    fn it_can_get_side() {
        let left = r#"#####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####.....
            #####....."#
            .parse::<BitView>()
            .unwrap();
        assert_eq!(
            left.get_side(&Direction::Right).collect::<Vec<_>>(),
            vec![Some(&false); 10]
        );
    }

    #[test]
    fn it_can_chop_off_sides() {
        let bottom = r#"##########
            #...#....#
            #...#....#
            #...#....#
            #...#....#
            ##########
            #...#....#
            #...#....#
            #...#....#
            ##########"#
            .parse::<BitView>()
            .unwrap();
        let expected = r#"...#....
...#....
...#....
...#....
########
...#....
...#....
...#...."#;

        assert_eq!(bottom.without_sides().to_string(), expected);
    }
}
