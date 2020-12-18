// use itertools::Itertools;

#[derive(Debug)]
struct Universe<T> {
    cubes: std::collections::HashSet<T>,
}

impl<T: std::fmt::Debug + Position + std::hash::Hash + std::cmp::Eq + Position<ReturnType = T>> Universe<T> {
    fn count(&self) -> usize {
        self.cubes.len()
    }

    fn tick(&mut self) {
        let mut influence_map = std::collections::HashMap::new();
        for pos in self.cubes.iter() {
            for other_pos in pos.surrounding_cubes() {
                *influence_map.entry(other_pos).or_insert(0) += 1;
            }
        }

        // remove dead
        self.cubes.retain(|pos| {
            match influence_map.get(pos) {
                Some(2) | Some(3) => { true},
                _ => false
            }
        });
        // add new life
        self.cubes.extend(influence_map.into_iter().filter_map(|(pos, count)| {
            match count {
                3 => Some(pos),
                _ => None
            }
        }));
    }
}

// impl std::fmt::Debug for Universe {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut cubes = self.cubes.clone();
//         cubes.sort_by_key(|pos| [pos.z, pos.x, pos.y]);
//         cubes.iter().group_by(|pos| pos.z).fold(String::new(), |memo, z_area| {
//         });
//         write!(f, "{}", result)
//     }
// }

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos3d {
    x: i32,
    y: i32,
    z: i32,
}

trait Position {
    type ReturnType;

    fn surrounding_cubes(&self) -> Vec<Self::ReturnType>;
}

impl std::fmt::Debug for Pos3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{},y:{},z:{}", self.x, self.y, self.z)
    }
}

impl Position for Pos3d {
    type ReturnType = Self;

    fn surrounding_cubes(&self) -> Vec<Self> {
        (-1..2).flat_map(|x| {
            (-1..2).flat_map(move |y| {
                (-1..2).filter_map(move |z| {
                    if x == 0 && y == 0 && z == 0 {
                        return None
                    }
                    Some(Pos3d { x: self.x + x, y: self.y + y, z: self.z + z })
                })
            })
        }).collect()
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos4d {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Position for Pos4d {
    type ReturnType = Self;

    fn surrounding_cubes(&self) -> Vec<Self> {
        (-1..2).flat_map(|x| {
            (-1..2).flat_map(move |y| {
                    (-1..2).flat_map(move |z| {
                        (-1..2).filter_map(move |w| {
                            if x == 0 && y == 0 && z == 0 && w == 0{
                                return None
                            }
                            Some(Pos4d { x: self.x + x, y: self.y + y, z: self.z + z, w: self.w + w })
                        })
                    })
            })
        }).collect()
    }
}

#[derive(Debug)]
enum Error {
    CouldntReadUniverse,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl std::str::FromStr for Universe<Pos3d> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().filter_map(move |(x, c)| {
                    match c {
                        '#' => Some(Pos3d {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                        }),
                        _ => None,
                    }
                })
            })
            .collect::<std::collections::HashSet<Pos3d>>();

        Ok(Universe { cubes })
    }
}

impl std::str::FromStr for Universe<Pos4d> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().filter_map(move |(x, c)| {
                    match c {
                        '#' => Some(Pos4d {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                            w: 0,
                        }),
                        _ => None,
                    }
                })
            })
            .collect::<std::collections::HashSet<Pos4d>>();

        Ok(Universe { cubes })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut universe: Universe<Pos3d> = input.parse()?;
    for _ in 0..6 {
        universe.tick();
    }
    println!("part1: {}", universe.count());

    let mut universe: Universe<Pos4d> = input.parse()?;
    for _ in 0..6 {
        universe.tick();
    }
    println!("part2: {}", universe.count());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let input = r#".#.
..#
###"#;
        let mut universe : Universe<Pos3d> = input.parse().unwrap();
        for _ in 0..6 {
            universe.tick();
        }
        assert_eq!(universe.count(), 112);
    }

    #[test]
    fn part2_works() {
        let input = r#".#.
..#
###"#;
        let mut universe : Universe<Pos4d> = input.parse().unwrap();
        for _ in 0..6 {
            universe.tick();
        }
        assert_eq!(universe.count(), 848);
    }
}
