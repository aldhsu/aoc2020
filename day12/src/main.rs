use std::str::FromStr;

enum Instruction {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug)]
enum Error {
    InstructionParseError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let int = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .map_err(|_| Error::InstructionParseError)?;
        Ok(
            match s.chars().next().ok_or(Error::InstructionParseError)? {
                'N' => N(int),
                'E' => E(int),
                'S' => S(int),
                'W' => W(int),
                'L' => L(int),
                'R' => R(int),
                'F' => F(int),
                _ => return Err(Error::InstructionParseError),
            },
        )
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

enum Hand {
    Left,
    Right,
}

impl Direction {
    const DIRECTIONS_IN_ORDER: [Direction; 4] =
        [Direction::N, Direction::E, Direction::S, Direction::W];
    const DIR_LEN: usize = Self::DIRECTIONS_IN_ORDER.len();
    fn turn(&self, degrees: i32, rotating: Hand) -> Direction {
        let ticks = degrees / 90;
        let current_pos = Self::DIRECTIONS_IN_ORDER
            .iter()
            .position(|dir| &dir == &self)
            .unwrap();
        let new_pos = match rotating {
            Hand::Left => current_pos as i32 - ticks,
            Hand::Right => current_pos as i32 + ticks,
        }
        .rem_euclid(Self::DIR_LEN as i32);
        Self::DIRECTIONS_IN_ORDER[new_pos as usize]
    }
}

struct Boat {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Boat {
    fn apply(&mut self, instruction: Instruction) -> Result<(), Error> {
        use Instruction::*;

        fn match_instruction(boat: &mut Boat, instruction: Instruction) {
            match instruction {
                N(val) => boat.y += val,
                E(val) => boat.x += val,
                S(val) => boat.y -= val,
                W(val) => boat.x -= val,
                L(degrees) => boat.direction = boat.direction.turn(degrees, Hand::Left),
                R(degrees) => boat.direction = boat.direction.turn(degrees, Hand::Right),
                _ => unreachable!(),
            }
        }

        match instruction {
            F(val) => match_instruction(
                self,
                match self.direction {
                    Direction::N => Instruction::N(val),
                    Direction::S => Instruction::S(val),
                    Direction::E => Instruction::E(val),
                    Direction::W => Instruction::W(val),
                },
            ),
            ins => match_instruction(self, ins),
        }

        Ok(())
    }
}

struct BoatWithWaypoint {
    x: i32,
    y: i32,
    direction: Direction,
    waypoint: Waypoint,
}

struct Waypoint {
    rel_x: i32,
    rel_y: i32,
}

impl BoatWithWaypoint {
    fn apply(&mut self, instruction: Instruction) -> Result<(), Error> {
        use Instruction::*;
        fn translate(mut degrees: i32, rotation: Hand, boat: &mut BoatWithWaypoint) {
            let x = boat.waypoint.rel_x;
            let y = boat.waypoint.rel_y;

            if let Hand::Left = rotation {
                degrees *= -1;
            }
            let quadrant = (degrees / 90).rem_euclid(4);

            let (rel_x, rel_y) = match quadrant {
                0 => (x, y),   // 1, 2
                1 => (y, -x),  // 2, -1
                2 => (-x, -y), // -1, -2
                3 => (-y, x),  // -2, 1
                _ => unreachable!(),
            };
            boat.waypoint.rel_x = rel_x;
            boat.waypoint.rel_y = rel_y;
        }

        match instruction {
            N(val) => self.waypoint.rel_y += val,
            E(val) => self.waypoint.rel_x += val,
            S(val) => self.waypoint.rel_y -= val,
            W(val) => self.waypoint.rel_x -= val,
            L(degrees) => translate(degrees, Hand::Left, self),
            R(degrees) => translate(degrees, Hand::Right, self),
            F(val) => {
                self.x += self.waypoint.rel_x * val;
                self.y += self.waypoint.rel_y * val;
            }
        }

        Ok(())
    }
}

fn plot_course(
    instructions: impl Iterator<Item = Result<Instruction, Error>>,
) -> Result<u32, Error> {
    let mut boat = Boat {
        x: 0,
        y: 0,
        direction: Direction::E,
    };
    for instruction in instructions {
        boat.apply(instruction?)?;
    }

    Ok((boat.x.abs() + boat.y.abs()) as u32)
}

fn plot_course_with_waypoint(
    instructions: impl Iterator<Item = Result<Instruction, Error>>,
) -> Result<u32, Error> {
    let mut boat = BoatWithWaypoint {
        x: 0,
        y: 0,
        direction: Direction::E,
        waypoint: Waypoint {
            rel_x: 10,
            rel_y: 1,
        },
    };

    for instruction in instructions {
        boat.apply(instruction?)?;
    }

    Ok((boat.x.abs() + boat.y.abs()) as u32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let instructions = input.lines().map(|line| line.parse::<Instruction>());
    let part1 = plot_course(instructions.clone())?;
    let part2 = plot_course_with_waypoint(instructions)?;
    println!("part1: {}", part1);
    println!("part1: {}", part2);
    Ok(())
}
