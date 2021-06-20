mod bitview;
mod coordinate;
mod direction;
mod error;
mod image;
mod map;

use crate::map::Map;
use image::Image;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let map: Map = input.parse()?;
    let corners = map.find_corners()?;
    println!(
        "part1: {}",
        corners.iter().fold(1_u64, |memo, num| *num as u64 * memo)
    );
    let non_dragon_count = map.filter_dragons()?;

    println!("part2: {}", non_dragon_count,);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_for_test() {
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
        assert_eq!(map.filter_dragons().unwrap(), 273);
    }
}
