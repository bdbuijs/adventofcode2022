use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, AddAssign, RangeInclusive, Sub, SubAssign},
};

use nom::{
    character::complete::char as nomchar,
    character::complete::{i8 as nomi8, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, points_vec) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let points = points_vec.into_iter().collect::<HashSet<_>>();
    let exposed_faces = points
        .iter()
        .map(|p| p.neighbours().filter(|n| !points.contains(n)).count())
        .sum::<usize>();
    exposed_faces.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, points_vec) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let (mut min, mut max) = (Point::MAX, Point::MIN);
    let points = points_vec
        .into_iter()
        .inspect(|p| {
            min = min.min(p);
            max = max.max(p);
        })
        .collect::<HashSet<_>>();
    let one = Point { x: 1, y: 1, z: 1 };
    max += one;
    min -= one;
    let range = PointRange::new(min, max);
    let mut outside_faces = 0;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(max);
    while let Some(point) = queue.pop_front() {
        point.neighbours_in_range(&range).for_each(|n| {
            if visited.contains(&n) {
                return;
            }
            if points.contains(&n) {
                outside_faces += 1;
            } else {
                queue.push_back(n);
                visited.insert(n);
            }
        })
    }
    outside_faces.to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Point {
    x: i8,
    y: i8,
    z: i8,
}

impl Point {
    const NEIGHBOURS: [Self; 6] = [
        Self { x: -1, y: 0, z: 0 },
        Self { x: 1, y: 0, z: 0 },
        Self { x: 0, y: -1, z: 0 },
        Self { x: 0, y: 1, z: 0 },
        Self { x: 0, y: 0, z: -1 },
        Self { x: 0, y: 0, z: 1 },
    ];

    const MAX: Self = Self {
        x: i8::MAX,
        y: i8::MAX,
        z: i8::MAX,
    };
    const MIN: Self = Self {
        x: i8::MIN,
        y: i8::MIN,
        z: i8::MIN,
    };

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, x) = nomi8(input)?;
        let (input, _) = nomchar(',')(input)?;
        let (input, y) = nomi8(input)?;
        let (input, _) = nomchar(',')(input)?;
        let (input, z) = nomi8(input)?;
        Ok((input, Self { x, y, z }))
    }

    fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        Self::NEIGHBOURS.iter().cloned().map(|n| *self + n)
    }

    fn neighbours_in_range<'a>(&'a self, range: &'a PointRange) -> impl Iterator<Item = Self> + 'a {
        self.neighbours().filter(|n| range.contains(n))
    }

    fn max(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    fn min(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

struct PointRange {
    x: RangeInclusive<i8>,
    y: RangeInclusive<i8>,
    z: RangeInclusive<i8>,
}

impl PointRange {
    fn new(min: Point, max: Point) -> Self {
        Self {
            x: min.x..=max.x,
            y: min.y..=max.y,
            z: min.z..=max.z,
        }
    }
    fn contains(&self, point: &Point) -> bool {
        self.x.contains(&point.x) && self.y.contains(&point.y) && self.z.contains(&point.z)
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, lines) = separated_list1(newline, Point::parse)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "64");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "58");
    }
}
