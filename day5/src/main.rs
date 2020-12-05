fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut all_seats = input
        .lines()
        .map(|line| {
            let seat: Seat = line.into();
            seat
        })
        .collect::<Vec<_>>();
    let max = all_seats
        .iter()
        .map(|seat| seat.id)
        .max()
        .expect("couldn't find max");
    println!("part1: {}", max);

    all_seats.sort_by(|a, b| a.id.cmp(&b.id));
    let result = all_seats
        .windows(2)
        .find(|window| {
            let mut iter = window.iter();
            let a = iter.next().expect("can't find first seat");
            let b = iter.next().expect("can't find second seat");
            (b.id - a.id) == 2
        })
        .expect("couldn't find a window that matched");

    println!("part2: {}", result.iter().next().unwrap().id + 1);
    Ok(())
}

#[derive(Debug)]
struct Seat {
    row: i16,
    column: i16,
    id: i32,
}

impl From<&str> for Seat {
    fn from(input: &str) -> Self {
        fn collect_bits(mut memo: i16, c: char) -> i16 {
            memo = memo << 1;
            memo |= match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!("unknown char to bit"),
            };
            memo
        }

        let row = input.chars().take(7).fold(0_i16, collect_bits);

        let column = input.chars().skip(7).fold(0, collect_bits);

        Self {
            row,
            column,
            id: row as i32 * 8 + column as i32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_string_to_seat() {
        let seat: Seat = "BFFFBBFRRR".into();
        // row 70, column 7, seat ID 567
        assert_eq!(seat.row, 70);
        assert_eq!(seat.column, 7);
        assert_eq!(seat.id, 567);
    }
}
