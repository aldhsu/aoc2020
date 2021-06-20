use crate::bitview::BitView;
use crate::coordinate::CartCoord;
use crate::direction::Direction;
use crate::error::Error;
use crate::Image;

type Tile = (u32, usize);

#[derive(Debug)]
pub struct CoordImageMap {
    pub inner: std::collections::HashMap<CartCoord, (u32, usize)>,
}

impl CoordImageMap {
    fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    fn get<'map>(&self, coordinate: &CartCoord, map: &'map Map) -> Option<(&'map Image, usize)> {
        let (id, rotation) = self.inner.get(coordinate)?;

        Some((
            map.images.get(
                *map.view_map.get(id)?, //index
            )?,
            *rotation,
        ))
    }

    fn insert(&mut self, coord: CartCoord, tile: Tile) -> Option<(u32, usize)> {
        self.inner.insert(coord, tile)
    }

    pub fn single_map(&self, map: &Map) -> Result<BitView, Error> {
        let min_x: isize = self
            .inner
            .keys()
            .min_by(|(x1, _), (x2, _)| x1.cmp(x2))
            .ok_or(Error::CannotPrint)?
            .0;
        let min_y = self
            .inner
            .keys()
            .min_by(|(_, y1), (_, y2)| y1.cmp(y2))
            .ok_or(Error::CannotPrint)?
            .1;
        let max_x = self
            .inner
            .keys()
            .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
            .ok_or(Error::CannotPrint)?
            .0;

        let cell_len = max_x - min_x + 1;
        let side_len = 8; // need to chop off the sides
        let side_count = cell_len as usize * side_len;

        let bits = (0..side_count)
            .flat_map(|y| {
                let map_y = (y / side_len) as isize + min_y;
                let view_y = y % side_len;

                (0..side_count).map(move |x| {
                    let map_x = (x / side_len) as isize + min_x;
                    let view_x = x % side_len;
                    let (image, rotation) = self
                        .get(&(map_x, map_y), map)
                        .ok_or(Error::CannotPrint)?;
                    let view = image.get_view(rotation).without_sides();

                    Ok(*view.get(&(view_x, view_y)).unwrap())
                })
            })
            .collect::<Result<Vec<bool>, Error>>()?;

        let side_len = (bits.len() as f32).sqrt() as usize;
        Ok(BitView { bits, side_len })
    }
}

pub struct Map {
    pub side_length: usize,
    pub images: Vec<Image>,
    pub view_map: std::collections::HashMap<u32, usize>,
}

fn move_dir((x, y): &CartCoord, direction: &Direction) -> CartCoord {
    match direction {
        Direction::Up => (*x, y - 1),
        Direction::Down => (*x, y + 1),
        Direction::Left => (x - 1, *y),
        Direction::Right => (x + 1, *y),
    }
}

impl Map {
    pub fn filter_dragons(&self) -> Result<u32, Error> {
        let dragon_pattern = r#"                  #
#    ##    ##    ###
 #  #  #  #  #  #   "#;

        let pattern = dragon_pattern
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if matches!(c, '#') {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<(usize, usize)>>();

        let corners = self.find_corners()?;
        let solved = self.solve_puzzle(corners)?;
        let single_map = solved.single_map(&self)?;
        let side_len = single_map.side_len;
        let total_xs = single_map.bits.iter().filter(|&&b| b).count();
        let maps = single_map.all_complements();

        let result = maps
            .iter()
            .find_map(|map| {
                let mut matches = std::collections::HashSet::new();
                for y in 0..side_len {
                    for x in 0..side_len {
                        let all_match = pattern.iter().all(|(offset_x, offset_y)| {
                            let new_x = x + offset_x;
                            let new_y = y + offset_y;
                            if matches.contains(&(new_x, new_y)) {
                                false
                            } else {
                                matches!(map.get(&(new_x, new_y)), Some(true))
                            }
                        });

                        if all_match {
                            pattern.iter().for_each(|(offset_x, offset_y)| {
                                matches.insert((x + offset_x, y + offset_y));
                            });
                        }
                    }
                }

                if !matches.is_empty() {
                    Some(matches)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::SolveFailed("couldn't find a dragon".to_string()))?;

        Ok((total_xs - result.len()) as u32)
    }
    fn side_map(&self) -> std::collections::HashMap<Vec<bool>, std::collections::HashSet<u32>> {
        let mut side_map: std::collections::HashMap<Vec<bool>, std::collections::HashSet<u32>> =
            std::collections::HashMap::new();

        for image in &self.images {
            for view in &image.views {
                let side = view
                    .get_side(&Direction::Up)
                    .map(|b| *b.unwrap())
                    .collect::<Vec<bool>>();
                side_map
                    .entry(side)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(image.id);
            }
        }

        side_map
    }
    pub fn find_corners(&self) -> Result<Vec<u32>, Error> {
        let side_map = self.side_map();

        Ok(side_map
            .values()
            .filter(|value| value.len() == 1)
            .fold(std::collections::HashMap::new(), |mut memo, ids| {
                *memo
                    .entry(*ids.iter().next().expect("couldn't get id val"))
                    .or_insert(0) += 1;
                memo
            })
            .iter()
            .filter_map(|(k, v)| if v >= &4 { Some(*k) } else { None })
            .collect())
    }

    pub fn solve_puzzle(&self, corners: Vec<u32>) -> Result<CoordImageMap, Error> {
        // start with first corner
        // go around each side and attach
        // add more to work array
        // finish when work array is empty
        let side_map = self.side_map();
        let mut current_map = CoordImageMap::new();
        let first_id = corners.into_iter().next().expect("no corners given");

        let mut seen: std::collections::HashSet<u32> = std::collections::HashSet::new();
        seen.insert(first_id);

        current_map.insert((0, 0), (first_id, 0));
        let first_image = &self.images[*self
            .view_map
            .get(&first_id)
            .expect("couldn't get corner id")];

        let mut work = first_image
            .sides(0)
            .into_iter()
            .map(|side| (side, (0, 0)))
            .collect::<Vec<((Vec<bool>, Direction), CartCoord)>>();

        while let Some(((side, direction), coord)) = work.pop() {
            let from_map = side_map
                .get(&side)
                .ok_or_else(|| Error::SolveFailed("can't get side".to_string()))?;

            let unseen = from_map.iter().find(|id| !seen.contains(id));

            if let Some(id) = unseen {
                seen.insert(*id);
                let new_coord = move_dir(&coord, &direction);
                let image = &self.images[*self
                    .view_map
                    .get(id)
                    .ok_or_else(|| Error::SolveFailed("couldn't get next id".to_string()))?];
                let rotation: usize = image
                    .get_rotation(&side, &direction)
                    .ok_or_else(|| Error::SolveFailed("Couldn't find matching side".to_string()))?;

                current_map.insert(new_coord, (*id, rotation));

                let sides = image
                    .sides(rotation)
                    .into_iter()
                    .map(|side| (side, new_coord));

                work.extend(sides);
            }
        }
        Ok(current_map)
    }
}

impl std::str::FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let images = input
            .trim()
            .split("\n\n")
            .map(|chunk| chunk.trim().parse::<Image>())
            .collect::<Result<Vec<_>, _>>()?;

        let side_length = (images.len() as f64).sqrt() as usize;

        let view_map = images
            .iter()
            .enumerate()
            .map(|(i, image)| (image.id, i))
            .collect();

        Ok(Map {
            side_length,
            images,
            view_map,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_make_a_single_map() {
        let input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
        let map = input.parse::<Map>().unwrap();
        let mut corners = map.find_corners().unwrap();
        corners.sort();
        assert_eq!(corners, vec![1171, 1951, 2971, 3079]);

        let solved = map.solve_puzzle(vec![1951]).unwrap();
        let single_map = solved.single_map(&map).unwrap();

        let expected_map = r#".####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.##..
#.#.##.###.#.##.##.#####
..##.###.####..#.####.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.######.
.###.###.#######..#####.
..##.#..#..#.#######.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#"#;
        assert!(single_map
            .all_complements()
            .iter()
            .map(|map| map.to_string())
            .find(|map| map == expected_map)
            .is_some());
    }
}
