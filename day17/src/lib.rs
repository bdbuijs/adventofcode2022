#![allow(unstable_name_collisions)]
// great if Itertools features are stabilised into std, but don't bitch at me until they are!
use std::collections::HashSet;

use itertools::Itertools;

use nom::{character::complete::one_of, multi::many1, IResult};

pub fn process_part1(input: &str) -> String {
    let (input, jets) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut jetstream = jets.iter().cycle().intersperse(&Jet::Down);
    let mut cave = Cave {
        width: 7,
        rocks_count: 2022,
        next_rock_idx: 0,
        rocks: HashSet::new(),
        highest_rock: 0,
    };
    while let Some(mut rock) = cave.next() {
        loop {
            if rock.push(&mut cave, jetstream.next().expect("infinite stream")) {
                break;
            }
        }
    }

    cave.highest_rock.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, jets) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut jetstream = jets.iter().cycle().intersperse(&Jet::Down).enumerate();
    let mut cave = Cave {
        width: 7,
        rocks_count: 1_000_000_000_000,
        next_rock_idx: 0,
        rocks: HashSet::new(),
        highest_rock: 0,
    };
    let cycle_length = jets.len() * 2;
    let mut extra = 0;
    let mut resting_positions: Vec<(u64, usize, u64, u64)> = Vec::new();
    while let Some(mut rock) = cave.next() {
        loop {
            let (jet_count, jet) = jetstream.next().expect("infinite stream");
            if rock.push(&mut cave, jet) {
                if extra == 0 && rock.typ == RockType::Underscore {
                    let tup = (
                        rock.bottom_left.x,
                        jet_count % cycle_length,
                        cave.rocks_count,
                        cave.highest_rock,
                    );
                    if let Some((_, _, previous_rock_count, previous_highest_rock)) =
                        resting_positions.iter().find(|&&previous_tup| {
                            previous_tup.0 == tup.0 && previous_tup.1 == tup.1
                        })
                    {
                        // we've seen this kinda thing before!
                        // it takes rockz to grow the stack by stack_size
                        let rockz = previous_rock_count - cave.rocks_count;
                        let stack_size = cave.highest_rock - previous_highest_rock;
                        extra = (cave.rocks_count / rockz) * stack_size;
                        cave.rocks_count %= rockz;
                    };
                    resting_positions.push(tup);
                }
                break;
            }
        }
    }

    (cave.highest_rock + extra).to_string()
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
    Down, // gravity implemented as a downward jet
}

impl Jet {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = one_of("<>")(input)?;
        let jet = match c {
            '<' => Self::Left,
            '>' => Self::Right,
            x => unreachable!("Unexpected jet character: {x}"),
        };
        Ok((input, jet))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    fn left_one(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right_one(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn down_one(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RockType {
    Underscore,
    // ####
    Plus,
    // .#.
    // ###
    // .#.
    ReverseL,
    // ..#
    // ..#
    // ###
    I,
    // #
    // #
    // #
    // #
    O,
    // ##
    // ##
}

impl RockType {
    fn height(&self) -> u64 {
        match self {
            Self::Underscore => 1,
            Self::Plus => 3,
            Self::ReverseL => 3,
            Self::I => 4,
            Self::O => 2,
        }
    }

    fn width(&self) -> u64 {
        match self {
            Self::Underscore => 4,
            Self::Plus => 3,
            Self::ReverseL => 3,
            Self::I => 1,
            Self::O => 2,
        }
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        fn points_iter(points: &[Point]) -> impl Iterator<Item = Point> + '_ {
            points.iter().cloned()
        }
        match self {
            RockType::Underscore => points_iter(&[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ]),
            RockType::Plus => points_iter(&[
                Point { x: 1, y: 2 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
            ]),
            RockType::ReverseL => points_iter(&[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ]),
            RockType::I => points_iter(&[
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ]),
            RockType::O => points_iter(&[
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
            ]),
        }
    }
}

impl TryFrom<u8> for RockType {
    type Error = InvalidEnumValueError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RockType::Underscore),
            1 => Ok(RockType::Plus),
            2 => Ok(RockType::ReverseL),
            3 => Ok(RockType::I),
            4 => Ok(RockType::O),
            x => Err(InvalidEnumValueError(x)),
        }
    }
}

#[derive(Debug)]
struct InvalidEnumValueError(u8);

impl std::fmt::Display for InvalidEnumValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid value for Rock: {}", self.0)
    }
}

impl std::error::Error for InvalidEnumValueError {}

#[derive(Debug)]

struct Rock {
    typ: RockType,
    bottom_left: Point,
}

impl Rock {
    fn new(type_idx: u8, highest_rock: u64) -> Self {
        let typ: RockType = type_idx.try_into().expect("Invalid type_idx");
        let bottom_left = Point::new(2, highest_rock + 4);
        Self { typ, bottom_left }
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.typ.points().map(|p| p + self.bottom_left)
    }

    /// returns true if rock has landed
    fn push(&mut self, cave: &mut Cave, jet: &Jet) -> bool {
        match jet {
            Jet::Down
                if self.bottom_left.y > 1
                    && self.points().all(|p| !cave.rocks.contains(&p.down_one())) =>
            {
                self.bottom_left = self.bottom_left.down_one();
            }
            Jet::Down => {
                cave.highest_rock = cave
                    .highest_rock
                    .max(self.bottom_left.y + self.typ.height() - 1);
                cave.rocks.extend(self.points());
                return true;
            }

            Jet::Left
                if self.bottom_left.x > 0
                    && self.points().all(|p| !cave.rocks.contains(&p.left_one())) =>
            {
                self.bottom_left = self.bottom_left.left_one();
            }

            Jet::Right
                if self.bottom_left.x + self.typ.width() < cave.width as u64
                    && self.points().all(|p| !cave.rocks.contains(&p.right_one())) =>
            {
                self.bottom_left = self.bottom_left.right_one();
            }
            _ => {}
        }
        false
    }
}

struct Cave {
    width: u8,
    rocks_count: u64,
    next_rock_idx: u8,
    rocks: HashSet<Point>,
    highest_rock: u64,
}

impl Iterator for Cave {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rocks_count == 0 {
            None
        } else {
            let next_rock = Rock::new(self.next_rock_idx, self.highest_rock);
            self.next_rock_idx = (self.next_rock_idx + 1) % 5;
            self.rocks_count -= 1;
            Some(next_rock)
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Jet>> {
    many1(Jet::parse)(input)
}

#[allow(dead_code)]
fn print_cave(cave: &Cave, rock: &Rock) {
    let rocks = rock.points().collect::<HashSet<_>>();
    (1..=(cave.highest_rock + 7)).rev().for_each(|y| {
        print!("|");
        (0..7).for_each(|x| {
            let point = Point::new(x, y);
            if cave.rocks.contains(&point) {
                print!("#");
            } else if rocks.contains(&point) {
                print!("@");
            } else {
                print!(".");
            }
        });
        println!("|");
    });
    println!("+-------+");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "3068");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "1514285714288");
    }
}
