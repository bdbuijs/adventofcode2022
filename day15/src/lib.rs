use std::{collections::HashSet, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nomi64, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str, row: i64) -> String {
    let (input, sensors) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let beacons_in_row = sensors
        .iter()
        .map(|s| s.closest_beacon.clone())
        .filter(|b| b.y == row)
        .collect::<HashSet<Beacon>>()
        .len();
    let ranges = sensors.iter().filter_map(|s| s.range_in_row(row)).collect();
    let non_beacons = count_coverage(ranges) - beacons_in_row as u64;
    non_beacons.to_string()
}

pub fn process_part2(input: &str, range: RangeInclusive<i64>) -> String {
    let (input, sensors) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let tuning_frequency = find_beacon_tuning_frequency(sensors, range);
    tuning_frequency.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beacon {
    x: i64,
    y: i64,
}

impl Beacon {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("x=")(input)?;
        let (input, x) = nomi64(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y) = nomi64(input)?;
        Ok((input, Self { x, y }))
    }
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    closest_beacon: Beacon,
}

impl Sensor {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Sensor at x=")(input)?;
        let (input, x) = nomi64(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y) = nomi64(input)?;
        let (input, _) = tag(": closest beacon is at ")(input)?;
        let (input, closest_beacon) = Beacon::parse(input)?;
        Ok((
            input,
            Self {
                x,
                y,
                closest_beacon,
            },
        ))
    }

    fn range_in_row(&self, row: i64) -> Option<RangeInclusive<i64>> {
        let distance_to_beacon = (self.x.abs_diff(self.closest_beacon.x)
            + self.y.abs_diff(self.closest_beacon.y)) as i64;
        let distance_to_row = row.abs_diff(self.y) as i64;
        if distance_to_row > distance_to_beacon {
            None
        } else {
            let x_remaining = distance_to_beacon - distance_to_row;
            Some((self.x - x_remaining)..=(self.x + x_remaining))
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    let (input, lines) = separated_list1(newline, Sensor::parse)(input)?;
    Ok((input, lines))
}

fn count_coverage(ranges: Vec<RangeInclusive<i64>>) -> u64 {
    let mut ranges = ranges;
    ranges.sort_by_key(|r| *r.start());
    let mut merged_ranges = vec![ranges[0].clone()];
    ranges.into_iter().skip(1).for_each(|r| {
        let last = merged_ranges
            .last_mut()
            .expect("There's at least one range");
        if (r.start() - 1) <= *last.end() {
            let end = *r.end().max(last.end());
            *last = *last.start()..=end;
        } else {
            merged_ranges.push(r);
        }
    });
    merged_ranges.into_iter().map(|r| r.count()).sum::<usize>() as u64
}

fn find_beacon_tuning_frequency(sensors: Vec<Sensor>, range: RangeInclusive<i64>) -> usize {
    'search: for row in range.clone() {
        let mut ranges = sensors
            .iter()
            .filter_map(|s| s.range_in_row(row))
            .collect::<Vec<_>>();
        ranges.sort_by_key(|r| *r.start());
        let start = *range.start().max(ranges[0].start())..=(*ranges[0].end());
        debug_assert!(start.end() <= range.end());
        let mut merged_ranges = vec![start];
        for r in ranges.into_iter().skip(1) {
            if r.start() > range.end() {
                break;
            }
            let last = merged_ranges
                .last_mut()
                .expect("There's at least one range");
            if (r.start() - 1) <= *last.end() {
                let end = *r.end().max(last.end());
                *last = (*last.start())..=end;
                if last.start() <= range.start() && last.end() >= range.end() {
                    continue 'search;
                }
            } else {
                merged_ranges.push(r);
            }
        }
        if merged_ranges.len() > 1 {
            return ((merged_ranges[0].end() + 1) as usize) * 4_000_000 + row as usize;
        }
    }
    unreachable!("Cannot find distress beacon")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input, 10);
        assert_eq!(result, "26");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input, 0..=20);
        assert_eq!(result, "56000011");
    }
}
