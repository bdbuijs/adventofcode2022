use std::{collections::HashMap, ops::Index};

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
    let password = state.password();
    password.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (cave, instructions)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut cube: Cube = cave.into();
    instructions
        .into_iter()
        .for_each(|instruction| cube.walk(instruction));
    let password = cube.state.password();
    password.to_string()
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

    fn turned(&self, instruction: Instruction) -> Self {
        let mut t = *self;
        t.turn(instruction);
        t
    }

    fn reverse(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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
        &self.tiles[index]
    }
}

#[derive(Debug, Clone)]
struct ZipCorner {
    start: Point,
    first_arm: (Direction, Direction), // Start direction of zipping, target direction when entering adjacent face
    second_arm: (Direction, Direction),
}

#[derive(Debug)]
struct Cube {
    tiles: Vec<Vec<Tile>>,
    edge_transformations: HashMap<State, State>,
    state: State,
}

impl Cube {
    fn walk(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::R | Instruction::L => self.state = self.state.turn(instruction),
            Instruction::Move(steps) => {
                (0..steps).for_each(|_| self.step());
            }
        }
    }

    fn step(&mut self) {
        if let Some(&new_state) = self.edge_transformations.get(&self.state) {
            match self.tiles[new_state.position] {
                Tile::Empty => unimplemented!("Shouldn't teleport off the edge!"),
                Tile::Open => self.state = new_state,
                Tile::Wall => {}
            }
        } else if let Some(new_point) = match self.state.direction {
            Direction::Right => {
                if (self.state.position.x + 1) < self.tiles[0].len() {
                    Some(Point::new(self.state.position.x + 1, self.state.position.y))
                } else {
                    None
                }
            }
            Direction::Down => {
                if (self.state.position.y + 1) < self.tiles.len() {
                    Some(Point::new(self.state.position.x, self.state.position.y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.state.position.x > 0 {
                    Some(Point::new(self.state.position.x - 1, self.state.position.y))
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.state.position.y > 0 {
                    Some(Point::new(self.state.position.x, self.state.position.y - 1))
                } else {
                    None
                }
            }
        } {
            match self.tiles[new_point] {
                Tile::Empty => {
                    unreachable!("Should transform before walking off the edge!")
                }
                Tile::Open => self.state.position = new_point,
                Tile::Wall => {}
            };
        }
    }
}

impl From<Cave> for Cube {
    fn from(value: Cave) -> Self {
        let state = State::new(value.start());
        let tiles = value.tiles;
        let mut corners = Vec::new();
        value.x_spans.windows(2).enumerate().for_each(|(y, w)| {
            let (left1, left2) = (w[0].0, w[1].0);
            match left1.cmp(&left2) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    let corner = ZipCorner {
                        start: Point::new(left2, y),
                        first_arm: (Direction::Left, Direction::Up),
                        second_arm: (Direction::Down, Direction::Right),
                    };
                    corners.push(corner);
                }
                std::cmp::Ordering::Greater => {
                    let corner = ZipCorner {
                        start: Point::new(left1, y + 1),
                        first_arm: (Direction::Left, Direction::Down),
                        second_arm: (Direction::Up, Direction::Right),
                    };
                    corners.push(corner);
                }
            }
            let (right1, right2) = (w[0].1, w[1].1);
            match right1.cmp(&right2) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    let corner = ZipCorner {
                        start: Point::new(right1, y + 1),
                        first_arm: (Direction::Right, Direction::Down),
                        second_arm: (Direction::Up, Direction::Left),
                    };
                    corners.push(corner);
                }
                std::cmp::Ordering::Greater => {
                    let corner = ZipCorner {
                        start: Point::new(right2, y),
                        first_arm: (Direction::Right, Direction::Up),
                        second_arm: (Direction::Down, Direction::Left),
                    };
                    corners.push(corner);
                }
            }
        });
        let zipper = Zipper::new(&tiles, corners);
        let edge_transformations = zipper.collect::<HashMap<_, _>>();

        Self {
            tiles,
            edge_transformations,
            state,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn password(self) -> usize {
        1000 * (self.position.y + 1) + 4 * (self.position.x + 1) + usize::from(self.direction)
    }
}

struct Zipper<'tiles> {
    tiles: &'tiles Vec<Vec<Tile>>,
    corners: Vec<ZipCorner>,
    current_corner: usize,
    #[allow(dead_code)] // for if I ever implement the edge case
    side_length: usize,
    first_arm_travel_direction: Direction,
    first_arm_into_direction: Direction,
    first_arm_from_direction: Direction,
    first_arm_current_point: Point,
    first_arm_iter: Box<dyn Iterator<Item = Point> + 'tiles>,
    second_arm_travel_direction: Direction,
    second_arm_into_direction: Direction,
    second_arm_from_direction: Direction,
    second_arm_iter: Box<dyn Iterator<Item = Point> + 'tiles>,
    second_arm_current_point: Point,
    residual_state_pair: Option<(State, State)>,
    yielded: usize,
    max: usize,
}

impl<'tiles> Zipper<'tiles> {
    fn new(tiles: &'tiles Vec<Vec<Tile>>, corners: Vec<ZipCorner>) -> Self {
        let width = tiles[0].len();
        let height = tiles.len();

        let side_length = {
            if width * 4 == height * 3 {
                height / 4
            } else if width * 3 == height * 4 {
                width / 4
            } else if width * 2 == height * 5 {
                height / 2
            } else if width * 5 == height * 2 {
                width / 2
            } else {
                unreachable!("Invalid tile dimensions for cube net")
            }
        };
        let max = side_length * 14;

        let first_corner = &corners[0];
        let start = first_corner.start;
        let first_arm_travel_direction = first_corner.first_arm.0;
        let first_arm_into_direction = first_corner.first_arm.1;
        let first_arm_from_direction = first_arm_into_direction.reverse();
        let first_arm_current_point = start;
        let second_arm_travel_direction = first_corner.second_arm.0;
        let second_arm_into_direction = first_corner.second_arm.1;
        let second_arm_from_direction = second_arm_into_direction.reverse();
        let second_arm_current_point = start;

        let first_arm_iter = Self::arm_iter(tiles, start, first_corner.first_arm);
        let second_arm_iter = Self::arm_iter(tiles, start, first_corner.second_arm);

        let current_corner = 0;
        let yielded = 0;
        let residual_state_pair = None;

        Self {
            tiles,
            corners,
            current_corner,
            side_length,
            first_arm_travel_direction,
            first_arm_from_direction,
            first_arm_into_direction,
            first_arm_current_point,
            first_arm_iter,
            second_arm_travel_direction,
            second_arm_from_direction,
            second_arm_into_direction,
            second_arm_current_point,
            second_arm_iter,
            residual_state_pair,
            yielded,
            max,
        }
    }

    fn arm_iter(
        tiles: &'tiles [Vec<Tile>],
        start: Point,
        arm: (Direction, Direction),
    ) -> Box<dyn Iterator<Item = Point> + 'tiles> {
        match arm.0 {
            Direction::Right => Box::new(
                ((start.x + 1)..tiles[0].len())
                    .map(move |x| Point::new(x, start.y))
                    .take_while(|p| tiles[p.y][p.x] != Tile::Empty),
            ),
            Direction::Down => Box::new(
                ((start.y + 1)..tiles.len())
                    .map(move |y| Point::new(start.x, y))
                    .take_while(|p| tiles[p.y][p.x] != Tile::Empty),
            ),
            Direction::Left => Box::new(
                (0..start.x)
                    .rev()
                    .map(move |x| Point::new(x, start.y))
                    .take_while(|p| tiles[p.y][p.x] != Tile::Empty),
            ),
            Direction::Up => Box::new(
                (0..start.y)
                    .rev()
                    .map(move |y| Point::new(start.x, y))
                    .take_while(|p| tiles[p.y][p.x] != Tile::Empty),
            ),
        }
    }
}

impl<'a> Iterator for Zipper<'a> {
    type Item = (State, State);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(states) = self.residual_state_pair.take() {
            self.yielded += 1;
            return Some(states);
        }

        if self.yielded == self.max {
            return None;
        }

        let (first, second) = match (self.first_arm_iter.next(), self.second_arm_iter.next()) {
            (None, None) => {
                // both at a corner
                self.current_corner += 1;
                if self.current_corner == self.corners.len() {
                    // edge case
                    unimplemented!("Couldn't be bothered to implement the edge case algorithm");
                    // in case I ever want to: start again from an inside corner, walk both 'pointers'
                    // out until an unzipped edge is reached, zip from there until completely zipped
                }
                let corner = &self.corners[self.current_corner];
                let start = corner.start;
                self.first_arm_travel_direction = corner.first_arm.0;
                self.first_arm_into_direction = corner.first_arm.1;
                self.first_arm_from_direction = self.first_arm_into_direction.reverse();
                self.first_arm_current_point = start;
                self.second_arm_travel_direction = corner.second_arm.0;
                self.second_arm_into_direction = corner.second_arm.1;
                self.second_arm_from_direction = self.second_arm_into_direction.reverse();
                self.second_arm_current_point = start;
                self.first_arm_iter = Self::arm_iter(self.tiles, start, corner.first_arm);
                self.second_arm_iter = Self::arm_iter(self.tiles, start, corner.second_arm);

                return self.next();
            }
            (None, Some(second)) => {
                let previous_first = self.first_arm_current_point;
                let turn = if let Some(_next_left) = next(
                    self.tiles,
                    previous_first,
                    self.first_arm_travel_direction.turned(Instruction::L),
                ) {
                    Instruction::L
                } else {
                    assert!(next(
                        self.tiles,
                        previous_first,
                        self.first_arm_travel_direction.turned(Instruction::R),
                    )
                    .is_some());
                    Instruction::R
                };
                self.first_arm_travel_direction.turn(turn);
                self.first_arm_from_direction.turn(turn);
                self.first_arm_into_direction.turn(turn);
                self.first_arm_iter = Self::arm_iter(
                    self.tiles,
                    previous_first,
                    (
                        self.first_arm_travel_direction,
                        self.first_arm_into_direction,
                    ),
                );
                (previous_first, second)
            }
            (Some(first), None) => {
                let previous_second = self.second_arm_current_point;
                let turn = if let Some(_next_left) = next(
                    self.tiles,
                    previous_second,
                    self.second_arm_travel_direction.turned(Instruction::L),
                ) {
                    Instruction::L
                } else {
                    assert!(next(
                        self.tiles,
                        previous_second,
                        self.second_arm_travel_direction.turned(Instruction::R),
                    )
                    .is_some());
                    Instruction::R
                };
                self.second_arm_travel_direction.turn(turn);
                self.second_arm_from_direction.turn(turn);
                self.second_arm_into_direction.turn(turn);
                self.second_arm_iter = Self::arm_iter(
                    self.tiles,
                    previous_second,
                    (
                        self.second_arm_travel_direction,
                        self.second_arm_into_direction,
                    ),
                );
                (first, previous_second)
            }
            (Some(first), Some(second)) => (first, second),
        };
        self.first_arm_current_point = first;
        self.second_arm_current_point = second;
        self.residual_state_pair = Some((
            State {
                position: second,
                direction: self.second_arm_from_direction,
            },
            State {
                position: first,
                direction: self.first_arm_into_direction,
            },
        ));
        self.yielded += 1;
        Some((
            State {
                position: first,
                direction: self.first_arm_from_direction,
            },
            State {
                position: second,
                direction: self.second_arm_into_direction,
            },
        ))
    }
}

impl Index<Point> for Vec<Vec<Tile>> {
    type Output = Tile;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.y][index.x]
    }
}

#[allow(clippy::ptr_arg)]
fn next(tiles: &Vec<Vec<Tile>>, point: Point, direction: Direction) -> Option<Point> {
    let next_point = match direction {
        Direction::Right => {
            if point.x + 1 >= tiles[0].len() {
                return None;
            }
            Point {
                x: point.x + 1,
                y: point.y,
            }
        }
        Direction::Down => {
            if point.y + 1 >= tiles.len() {
                return None;
            }
            Point {
                x: point.x,
                y: point.y + 1,
            }
        }
        Direction::Left => {
            if point.x == 0 {
                return None;
            }
            Point {
                x: point.x - 1,
                y: point.y,
            }
        }
        Direction::Up => {
            if point.y == 0 {
                return None;
            }
            Point {
                x: point.x,
                y: point.y - 1,
            }
        }
    };
    if tiles[next_point] == Tile::Empty {
        None
    } else {
        Some(next_point)
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
        assert_eq!(result, "6032");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "5031");
    }
}
