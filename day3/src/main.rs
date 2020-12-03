fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let width = input.lines().next().expect("coudln't find first line").chars().count();
    let height = input.lines().count();

    let map : Vec<bool> = input.lines().flat_map(|line| {
        line.chars().map(|c| c == '#')
    }).collect();

    let horizontal_motion = 3;
    let mut tree_count = 0;
    let mut current_y = 0;
    let offset = width + horizontal_motion;
    loop {
        current_y += 1;
        if current_y > height { break }
        let index = (offset * current_y) % width + current_y * width;
        dbg!("{}", index);
        dbg!("{}", index % width);
        match map.get(index) {
            Some(true) => tree_count += 1,
            None => break,
            _ => continue,
        }
    }
    println!("part1: {}", tree_count);
    Ok(())
}
