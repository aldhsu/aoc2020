use crate::bitview::BitView;
use crate::direction::Direction;
use crate::error::Error;
#[derive(Debug)]
pub struct Image {
    pub views: Vec<BitView>,
    pub id: u32,
}

impl Image {
    pub fn get_view(&self, rotation: usize) -> &BitView {
        &self.views[rotation]
    }

    pub fn sides(&self, rotation: usize) -> Vec<(Vec<bool>, Direction)> {
        let view = &self.views[rotation];

        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .map(|direction| {
            (
                view.get_side(direction).map(|b| *b.unwrap()).collect(),
                *direction,
            )
        })
        .collect()
    }

    pub fn get_rotation(&self, side: &[bool], direction: &Direction) -> Option<usize> {
        let op_dir = direction.opposite();

        self.views.iter().enumerate().find_map(|(i, view)| {
            if side.iter().eq(view.get_side(&op_dir).map(|b| b.unwrap())) {
                Some(i)
            } else {
                None
            }
        })
    }
}

impl std::str::FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_and_bits = s.splitn(2, ":\n");
        let id = num_and_bits
            .next()
            .ok_or_else(|| Error::NotEnoughParts("tried to get num part".to_string()))?
            .split(' ')
            .nth(1)
            .ok_or_else(|| Error::NotEnoughParts(format!("tried to get id from {}", s)))?
            .parse::<u32>()
            .map_err(|_| Error::ParseError)?;

        let bits = num_and_bits
            .next()
            .ok_or_else(|| Error::NotEnoughParts("tried to get bits".to_string()))?
            .parse::<BitView>()?;

        Ok(Self {
            id,
            views: bits.all_complements(),
        })
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.views.first().unwrap())
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Image {}

#[cfg(test)]
mod test {
    use super::*;
}
