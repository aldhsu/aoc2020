#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn coordinates_for_side(&self, side_len: usize) -> impl Iterator<Item = (usize, usize)> {
        let x: Option<usize> = match self {
            Direction::Left => Some(0),
            Direction::Right => Some(side_len - 1),
            Direction::Up => None,
            Direction::Down => None,
        };

        let y: Option<usize> = match self {
            Direction::Left => None,
            Direction::Right => None,
            Direction::Up => Some(0),
            Direction::Down => Some(side_len - 1),
        };
        (0..side_len).map(move |i| match (&x, &y) {
            (None, Some(y)) => (i, *y),
            (Some(x), None) => (*x, i),
            _ => unreachable!(),
        })
    }
}
