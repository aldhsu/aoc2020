struct Map {
    map: Vec<bool>,
    height: usize,
    width: usize,
}

impl Map {
    fn count_trees_on_journey(&self, horizontal_motion: usize, down: usize) -> usize {
        let mut tree_count = 0;
        let mut current_y = 0;

        loop {
            current_y += down;
            if current_y > self.height { break }
            let index = (horizontal_motion * current_y) % self.width + current_y * self.width;
            match self.map.get(index) {
                Some(true) => tree_count += 1,
                None => break,
                _ => continue,
            }
        }
        tree_count
    }
}

fn part1(map: &Map) {
    let tree_count = map.count_trees_on_journey(3, 1);
    println!("part1: {}", tree_count);
}

fn part2(map: &Map) {
    let result = [
        map.count_trees_on_journey(1, 1),
        map.count_trees_on_journey(3, 1),
        map.count_trees_on_journey(5, 1),
        map.count_trees_on_journey(7, 1),
        map.count_trees_on_journey(1, 2),
    ].iter().fold(1, std::ops::Mul::mul);

    println!("part1: {:?}", result);

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let width = input.lines().next().expect("coudln't find first line").chars().count();
    let height = input.lines().count();

    let map : Vec<bool> = input.lines().flat_map(|line| {
        line.chars().map(|c| c == '#')
    }).collect();
    let map = Map {
        map,
        width,
        height,
    };

    part1(&map);
    part2(&map);
    Ok(())
}
