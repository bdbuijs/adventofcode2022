use std::ops::Index;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of, u16 as nomu16},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (cave, instructions)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let start = cave.start();
    let mut state = State::new(start);
    instructions
        .into_iter()
        .for_each(|instruction| state = cave.walk(state, instruction));
    let password =
        1000 * (state.position.y + 1) + 4 * (state.position.x + 1) + usize::from(state.direction);
    password.to_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Open,
    Wall,
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = one_of(" .#")(input)?;
        let tile = match c {
            ' ' => Self::Empty,
            '.' => Self::Open,
            '#' => Self::Wall,
            x => unreachable!("Invalid character for Tile: {x}"),
        };
        Ok((input, tile))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    R,
    L,
    Move(usize),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        if let Ok((input, turn)) = one_of::<&str, &str, ()>("LR")(input) {
            let instruction = match turn {
                'L' => Self::L,
                'R' => Self::R,
                x => unreachable!("Invalid character for instruction: {x}"),
            };
            return Ok((input, instruction));
        }
        let (input, steps) = nomu16(input)?;
        Ok((input, Self::Move(steps as usize)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::R => *self = (usize::from(*self) + 1).into(),
            Instruction::L => *self = ((usize::from(*self) + 3) % 4).into(),
            Instruction::Move(_) => {}
        }
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 | 4.. => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug)]
struct Cave {
    tiles: Vec<Vec<Tile>>,
    y_spans: Vec<(usize, usize)>,
    x_spans: Vec<(usize, usize)>,
}

impl Cave {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, mut tiles) =
            terminated(separated_list1(newline, parse_tile_row), tag("\n\n"))(input)?;
        let width = tiles
            .iter()
            .map(|row| row.len())
            .max()
            .expect("There's at least one row");
        let height = tiles.len();
        tiles
            .iter_mut()
            .for_each(|row| row.resize_with(width, || Tile::Empty));
        assert!(tiles.iter().all(|row| row.len() == width));
        let x_spans = tiles
            .iter()
            .map(|row| {
                let first = row
                    .iter()
                    .enumerate()
                    .find_map(
                        |(x, tile)| {
                            if tile != &Tile::Empty {
                                Some(x)
                            } else {
                                None
                            }
                        },
                    )
                    .expect("There are no empty rows");
                let last = if let Some(x) =
                    row.iter()
                        .enumerate()
                        .skip(first + 1)
                        .find_map(|(x, tile)| {
                            if tile == &Tile::Empty {
                                Some(x - 1)
                            } else {
                                None
                            }
                        }) {
                    x
                } else {
                    width - 1
                };
                (first, last)
            })
            .collect();
        let y_spans = (0..width)
            .map(|x| {
                let first = (0..height)
                    .find(|&y| tiles[y][x] != Tile::Empty)
                    .expect("There are no empty columns");
                let last =
                    if let Some(y) = ((first + 1)..height).find(|&y| tiles[y][x] == Tile::Empty) {
                        y - 1
                    } else {
                        height - 1
                    };
                (first, last)
            })
            .collect();
        Ok((
            input,
            Self {
                tiles,
                y_spans,
                x_spans,
            },
        ))
    }

    fn start(&self) -> Point {
        let x = self.tiles[0]
            .iter()
            .enumerate()
            .find_map(|(i, tile)| match tile {
                Tile::Empty => None,
                Tile::Open => Some(i),
                Tile::Wall => unreachable!("We better not start on a wall..."),
            })
            .expect("We gotta start somewhere!");
        Point::new(x, 0)
    }

    fn walk(&self, mut state: State, instruction: Instruction) -> State {
        match instruction {
            Instruction::R | Instruction::L => state.turn(instruction),
            Instruction::Move(steps) => {
                if let Some(new_position) = self.steps(&state).take(steps).last() {
                    state.position = new_position;
                }
                state
            }
        }
    }

    fn steps<'a>(&'a self, state: &'a State) -> impl Iterator<Item = Point> + 'a {
        let iterator: Box<dyn Iterator<Item = Point>> = match state.direction {
            Direction::Right => Box::new({
                let start_x = state.position.x;
                let y = state.position.y;
                let (start, end) = self.x_spans[y];
                (start_x..=end)
                    .skip(1)
                    .chain((start..=end).cycle())
                    .map(move |x| Point { x, y })
            }),
            Direction::Down => Box::new({
                let x = state.position.x;
                let start_y = state.position.y;
                let (start, end) = self.y_spans[x];
                (start_y..=end)
                    .skip(1)
                    .chain((start..=end).cycle())
                    .map(move |y| Point { x, y })
            }),
            Direction::Left => Box::new({
                let start_x = state.position.x;
                let y = state.position.y;
                let (start, end) = self.x_spans[y];
                (start..=start_x)
                    .rev()
                    .skip(1)
                    .chain((start..=end).rev().cycle())
                    .map(move |x| Point { x, y })
            }),
            Direction::Up => Box::new({
                let x = state.position.x;
                let start_y = state.position.y;
                let (start, end) = self.y_spans[x];
                (start..=start_y)
                    .rev()
                    .skip(1)
                    .chain((start..=end).rev().cycle())
                    .map(move |y| Point { x, y })
            }),
        };
        iterator.take_while(|&p| self[p] != Tile::Wall)
    }
}

impl Index<Point> for Cave {
    type Output = Tile;

    fn index(&self, index: Point) -> &Self::Output {
        &self.tiles[index.y][index.x]
    }
}

struct Cube {
    // stuff
}

impl Default for Cube {
    fn default() -> Self {
        Self {}
    }
}

impl From<Cave> for Cube {
    fn from(value: Cave) -> Self {
        let tiles = value.tiles;
        let width = tiles[0].len();
        let height = tiles.len();

        Default::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    position: Point,
    direction: Direction,
}

impl State {
    fn new(start: Point) -> Self {
        Self {
            position: start,
            direction: Direction::Right,
        }
    }

    fn turn(mut self, instruction: Instruction) -> Self {
        self.direction.turn(instruction);
        self
    }
}

fn parse_input(input: &str) -> IResult<&str, (Cave, Vec<Instruction>)> {
    let (input, cave) = Cave::parse(input)?;
    let (input, instructions) = many1(Instruction::parse)(input)?;
    Ok((input, (cave, instructions)))
}

fn parse_tile_row(input: &str) -> IResult<&str, Vec<Tile>> {
    let (input, row) = many1(Tile::parse)(input)?;
    Ok((input, row))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        3_isize.rem_euclid(3);
        assert_eq!(result, "6032");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "");
    }
}
