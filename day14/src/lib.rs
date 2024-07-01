use std::fmt::Write;

use nom::{
    bytes::complete::tag,
    character::complete::{char as nomchar, digit1, newline},
    error::{ErrorKind, ParseError},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, lines) = parse_input(input).unwrap();
    debug_assert!(input.is_empty());
    lines.iter().for_each(|line| {
        assert!(line
            .windows(2)
            .all(|w| { w[0].0 == w[1].0 || w[0].1 == w[1].1 }))
    });
    let mut cave = vec![vec![Cave::Air; 540]; 170];
    // fill cave with rocks
    lines.iter().for_each(|line| {
        line.windows(2).for_each(|w| {
            let ((x1, y1), (x2, y2)) = (w[0], w[1]);
            let ((x1, x2), (y1, y2)) = ((x1.min(x2), x1.max(x2)), (y1.min(y2), y1.max(y2)));
            if x1 == x2 {
                let x = x1;
                #[allow(clippy::needless_range_loop)] // symmetry trumps idioms, clippy!
                for y in y1..=y2 {
                    cave[y][x] = Cave::Rock;
                }
            } else if y1 == y2 {
                let y = y1;
                for x in x1..=x2 {
                    cave[y][x] = Cave::Rock;
                }
            } else {
                unreachable!("No diagonal walls allowed!")
            }
        })
    });
    // simulate sand until some sand drops
    let mut falling_sand = FallingSand::new();
    loop {
        if falling_sand.fall(&mut cave) {
            break;
        }
    }
    let resting_sand = cave
        .iter()
        .map(|row| row.iter().filter(|c| c == &&Cave::Sand).count())
        .sum::<usize>();
    // count the sand left
    resting_sand.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, lines) = parse_input(input).unwrap();
    debug_assert!(input.is_empty());
    lines.iter().for_each(|line| {
        assert!(line
            .windows(2)
            .all(|w| { w[0].0 == w[1].0 || w[0].1 == w[1].1 }))
    });
    let mut cave = vec![vec![Cave::Air; 1000]; 170];
    // fill cave with rocks
    let mut max_y = 0;
    lines.iter().for_each(|line| {
        line.windows(2).for_each(|w| {
            let ((x1, y1), (x2, y2)) = (w[0], w[1]);
            let ((x1, x2), (y1, y2)) = ((x1.min(x2), x1.max(x2)), (y1.min(y2), y1.max(y2)));
            max_y = max_y.max(y2);
            if x1 == x2 {
                let x = x1;
                #[allow(clippy::needless_range_loop)] // symmetry trumps idioms, clippy!
                for y in y1..=y2 {
                    cave[y][x] = Cave::Rock;
                }
            } else if y1 == y2 {
                let y = y1;
                for x in x1..=x2 {
                    cave[y][x] = Cave::Rock;
                }
            } else {
                unreachable!("No diagonal walls allowed!")
            }
        })
    });
    cave[max_y + 2].fill(Cave::Rock);

    // simulate sand until some sand drops
    let mut falling_sand = FallingSand::new();
    loop {
        falling_sand.fall(&mut cave);
        if cave[0][500] == Cave::Sand {
            break;
        }
    }
    let resting_sand = cave
        .iter()
        .map(|row| row.iter().filter(|c| c == &&Cave::Sand).count())
        .sum::<usize>();
    // count the sand left
    resting_sand.to_string()
}

type Line = Vec<(usize, usize)>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cave {
    Air,
    Rock,
    Sand,
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cave::Air => '.',
            Cave::Rock => '#',
            Cave::Sand => 'o',
        };
        f.write_char(c)
    }
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

struct FallingSand {
    x: usize,
    y: usize,
}

impl FallingSand {
    fn new() -> Self {
        Self { x: 500, y: 0 }
    }

    fn reset(&mut self) {
        self.x = 500;
        self.y = 0;
    }

    fn fall(&mut self, cave: &mut [Vec<Cave>]) -> bool {
        if self.y + 1 == cave.len() {
            return true;
        }
        if cave[self.y + 1][self.x] == Cave::Air {
            self.y += 1;
            false
        } else if cave[self.y + 1][self.x - 1] == Cave::Air {
            self.y += 1;
            self.x -= 1;
            false
        } else if cave[self.y + 1][self.x + 1] == Cave::Air {
            self.y += 1;
            self.x += 1;
            false
        } else {
            cave[self.y][self.x] = Cave::Sand;
            self.reset();
            false
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = separated_list1(tag(" -> "), parse_coordinate)(input)?;
    Ok((input, line))
}

fn parse_coordinate(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, x) = parse_usize(input)?;
    let (input, _) = nomchar(',')(input)?;
    let (input, y) = parse_usize(input)?;

    Ok((input, (x, y)))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, num) = digit1(input)?;
    let num = num
        .parse::<usize>()
        .map_err(|_| nom::Err::Error(ParseError::from_error_kind(input, ErrorKind::Digit)))?;
    Ok((input, num))
}

#[allow(dead_code)]
fn print_cave_rect(cave: &[Vec<Cave>], topleft: (usize, usize), width: usize, height: usize) {
    let (x, y) = topleft;
    (y..=(y + height)).for_each(|cur_y| {
        (x..=(x + width)).for_each(|cur_x| print!("{}", &cave[cur_y][cur_x]));
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "24");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "93");
    }
}
