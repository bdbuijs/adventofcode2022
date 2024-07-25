use std::collections::HashSet;

use once_cell::sync::OnceCell;

static WIDTH: OnceCell<usize> = OnceCell::new();
static HEIGHT: OnceCell<usize> = OnceCell::new();

pub fn process_part1(input: &str) -> String {
    let mut blizzards = parse_input(input);
    let mut elves: HashSet<Point> = HashSet::new();
    let mut next_elves = HashSet::new();
    let target = Point::new(WIDTH.get().unwrap() - 1, HEIGHT.get().unwrap() - 1);
    let start = Point::default();
    for round in 1.. {
        if elves.contains(&target) {
            return round.to_string();
        }
        assert!(next_elves.is_empty());
        next_elves.insert(start);
        elves
            .drain()
            .for_each(|elf| next_elves.extend(elf.next_positions()));
        // move the blizzards and kill all elves in the wrong positions
        blizzards.iter_mut().for_each(|b| {
            b.onward();
            next_elves.remove(&b.position);
        });
        std::mem::swap(&mut elves, &mut next_elves);
    }
    unreachable!()
}

pub fn process_part2(input: &str) -> String {
    let mut blizzards = parse_input(input);
    let mut elves: HashSet<Point> = HashSet::new();
    let mut next_elves = HashSet::new();
    let mut target = Point::new(WIDTH.get().unwrap() - 1, HEIGHT.get().unwrap() - 1);
    let mut start = Point::default();
    let mut targets = [start, target].into_iter();
    for round in 1.. {
        if elves.contains(&target) {
            if let Some(new_target) = targets.next() {
                start = target;
                target = new_target;
                elves.clear();
            } else {
                return round.to_string();
            }
        }
        assert!(next_elves.is_empty());
        next_elves.insert(start);
        elves
            .drain()
            .for_each(|elf| next_elves.extend(elf.next_positions()));
        // move the blizzards and kill all elves in the wrong positions
        blizzards.iter_mut().for_each(|b| {
            b.onward();
            next_elves.remove(&b.position);
        });
        std::mem::swap(&mut elves, &mut next_elves);
    }
    unreachable!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_char(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next_positions(&self) -> impl Iterator<Item = Self> {
        let x_end = *WIDTH.get().unwrap() - 1;
        let y_end = *HEIGHT.get().unwrap() - 1;
        [
            if self.x > 0 {
                Some(Point::new(self.x - 1, self.y))
            } else {
                None
            },
            if self.y > 0 {
                Some(Point::new(self.x, self.y - 1))
            } else {
                None
            },
            if self.x < x_end {
                Some(Point::new(self.x + 1, self.y))
            } else {
                None
            },
            if self.y < y_end {
                Some(Point::new(self.x, self.y + 1))
            } else {
                None
            },
            Some(*self),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Blizzard {
    direction: Direction,
    position: Point,
}

impl Blizzard {
    fn onward(&mut self) {
        let new_position = match self.direction {
            Direction::Up => {
                let height = HEIGHT.get().expect("Parsing has already been done");
                Point::new(self.position.x, (self.position.y + height - 1) % height)
            }
            Direction::Down => {
                let height = HEIGHT.get().expect("Parsing has already been done");
                Point::new(self.position.x, (self.position.y + 1) % height)
            }
            Direction::Left => {
                let width = WIDTH.get().expect("Parsing has already been done");
                Point::new((self.position.x + width - 1) % width, self.position.y)
            }
            Direction::Right => {
                let width = WIDTH.get().expect("Parsing has already been done");
                Point::new((self.position.x + 1) % width, self.position.y)
            }
        };
        self.position = new_position;
    }

    fn as_vizzard(&self) -> BlizzardVizzard {
        BlizzardVizzard::Single(self.direction)
    }
}

#[derive(Clone)]
enum BlizzardVizzard {
    Nothing,
    Single(Direction),
    Multiple(usize),
}

impl BlizzardVizzard {
    fn print(&self) {
        match self {
            BlizzardVizzard::Single(dir) => print!("{}", dir.as_char()),
            BlizzardVizzard::Multiple(n) => print!("{n}"),
            BlizzardVizzard::Nothing => print!("."),
        }
    }

    fn increase(&mut self, blizzard: &Blizzard) {
        match self {
            BlizzardVizzard::Nothing => *self = blizzard.as_vizzard(),
            BlizzardVizzard::Single(_) => *self = Self::Multiple(2),
            BlizzardVizzard::Multiple(n) => *self = Self::Multiple(*n + 1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Blizzard> {
    let _ = HEIGHT.set(input.lines().count() - 2);
    let _ = WIDTH.set(input.lines().next().expect("Field has multiple rows").len() - 2);
    input
        .lines()
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                match c {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None,
                }
                .map(|direction| Blizzard {
                    direction,
                    position: Point::new(x - 1, y),
                })
            })
        })
        .collect()
}

#[allow(dead_code)]
fn print_blizzards(blizzards: &[Blizzard]) {
    let mut field =
        vec![vec![BlizzardVizzard::Nothing; *WIDTH.get().unwrap()]; *HEIGHT.get().unwrap()];
    blizzards
        .iter()
        .for_each(|b| field[b.position.y][b.position.x].increase(b));
    field.iter().for_each(|row| {
        row.iter().for_each(|bv| bv.print());
        println!()
    });
    println!("----------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "18");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "54");
    }
}
