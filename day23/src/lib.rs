use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

const AROUND: [Elf; 8] = [
    Elf { x: -1, y: -1 },
    Elf { x: 0, y: -1 },
    Elf { x: 1, y: -1 },
    Elf { x: -1, y: 0 },
    Elf { x: 1, y: 0 },
    Elf { x: -1, y: 1 },
    Elf { x: 0, y: 1 },
    Elf { x: 1, y: 1 },
];

pub fn process_part1(input: &str) -> String {
    let mut elves = parse_input(input);
    let mut intentions = HashMap::new();
    for i in 0..10 {
        let directions = DIRECTIONS.iter().cycle().skip(i % 4).take(4);
        elves.iter().for_each(|elf| {
            if !elf.gonna_move(&elves) {
                intentions.entry(*elf).or_insert_with(Vec::new).push(*elf);
                return; // continue
            }
            for direction in directions.clone() {
                if let Some(new_elf) = elf.consider(direction, &elves) {
                    intentions
                        .entry(new_elf)
                        .or_insert_with(Vec::new)
                        .push(*elf);
                    return;
                }
            }
            intentions.entry(*elf).or_insert_with(Vec::new).push(*elf);
        });
        elves.clear();
        intentions.drain().for_each(|(new_elf, mut old_elves)| {
            if old_elves.len() == 1 {
                elves.insert(new_elf);
            } else {
                elves.extend(old_elves.drain(..));
            }
        });
    }
    let ground = count_ground(elves);
    ground.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut elves = parse_input(input);
    let mut previous_elves = HashSet::new();
    let mut intentions = HashMap::new();
    for i in 0.. {
        let directions = DIRECTIONS.iter().cycle().skip(i % 4).take(4);
        elves.iter().for_each(|elf| {
            if !elf.gonna_move(&elves) {
                intentions.entry(*elf).or_insert_with(Vec::new).push(*elf);
                return; // continue
            }
            for direction in directions.clone() {
                if let Some(new_elf) = elf.consider(direction, &elves) {
                    intentions
                        .entry(new_elf)
                        .or_insert_with(Vec::new)
                        .push(*elf);
                    return;
                }
            }
            intentions.entry(*elf).or_insert_with(Vec::new).push(*elf);
        });
        previous_elves.clear();
        std::mem::swap(&mut elves, &mut previous_elves);
        intentions.drain().for_each(|(new_elf, mut old_elves)| {
            if old_elves.len() == 1 {
                elves.insert(new_elf);
            } else {
                elves.extend(old_elves.drain(..));
            }
        });
        if elves == previous_elves {
            return (i + 1).to_string();
        }
    }
    unreachable!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Elf {
    x: isize,
    y: isize,
}

impl Elf {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn consider(&self, direction: &Direction, elves: &HashSet<Elf>) -> Option<Self> {
        let steps = match direction {
            Direction::North => [
                Elf { x: -1, y: -1 },
                Elf { x: 0, y: -1 },
                Elf { x: 1, y: -1 },
            ],
            Direction::South => [Elf { x: -1, y: 1 }, Elf { x: 0, y: 1 }, Elf { x: 1, y: 1 }],
            Direction::East => [Elf { x: 1, y: -1 }, Elf { x: 1, y: 0 }, Elf { x: 1, y: 1 }],
            Direction::West => [
                Elf { x: -1, y: -1 },
                Elf { x: -1, y: 0 },
                Elf { x: -1, y: 1 },
            ],
        };
        if steps.iter().all(|step| !elves.contains(&(*self + *step))) {
            Some(*self + steps[1])
        } else {
            None
        }
    }

    fn gonna_move(&self, elves: &HashSet<Elf>) -> bool {
        !AROUND.iter().all(|step| !elves.contains(&(*self + *step)))
    }
}

impl Add<Elf> for Elf {
    type Output = Self;

    fn add(self, rhs: Elf) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_input(input: &str) -> HashSet<Elf> {
    let (mut x, mut y) = (0_isize, 0_isize);
    input
        .chars()
        .filter_map(|c| {
            x += 1;
            match c {
                '.' => None,
                '#' => Some(Elf::new(x, y)),
                '\n' => {
                    x = 0;
                    y += 1;
                    None
                }
                x => unreachable!("Invalid character: {x}"),
            }
        })
        .collect::<HashSet<_>>()
}

fn count_ground(elves: HashSet<Elf>) -> usize {
    let (x_min, y_min, x_max, y_max) = elves.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(x_min, y_min, x_max, y_max), el| {
            (
                x_min.min(el.x),
                y_min.min(el.y),
                x_max.max(el.x),
                y_max.max(el.y),
            )
        },
    );
    let width = x_min.abs_diff(x_max) + 1;
    let height = y_min.abs_diff(y_max) + 1;
    width * height - elves.len()
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<Elf>) {
    let (x_min, y_min, x_max, y_max) = elves.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(x_min, y_min, x_max, y_max), el| {
            (
                x_min.min(el.x),
                y_min.min(el.y),
                x_max.max(el.x),
                y_max.max(el.y),
            )
        },
    );
    println!("-------------------------------------");
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if elves.contains(&Elf { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("-------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "110");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "20");
    }
}
