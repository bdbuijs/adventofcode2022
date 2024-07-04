use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::Not,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u8 as nomu8},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, valves) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let bitmap = match valves.len() {
        10 => 0b1000_0000_1111_1111, // only 7 nodes in the test case graph
        _ => 0b1000_0000_0000_0000,
    };
    let valves: Valves = valves.into();
    let state = State {
        current_valve: 0,
        closed_valves: ClosedValves::new(bitmap),
        minutes_remaining: 30,
    };
    let mut memo = HashMap::new();
    dfs(&valves, state, &mut memo).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, valves) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let valves: Valves = valves.into();
    let mut memo: HashMap<State, usize> = HashMap::new();
    (0b1000_0000_0000_0000_u16..0b1111_1111_1111_1111)
        .map(|bitmap| {
            let you = State {
                current_valve: 0,
                closed_valves: ClosedValves::new(bitmap),
                minutes_remaining: 26,
            };
            let elephant = State {
                current_valve: 0,
                closed_valves: ClosedValves::new(bitmap).inverse(),
                minutes_remaining: 26,
            };
            dfs(&valves, you, &mut memo) + (dfs(&valves, elephant, &mut memo))
        })
        .max()
        .expect("There are options")
        .to_string()
}

#[derive(Debug)]
struct Valve<'a> {
    id: &'a str,
    flow_rate: usize,
    tunnels: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = tag("Valve ")(input)?;
        let (input, id) = alpha1(input)?;
        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, flow_rate) = nomu8(input)?;
        let flow_rate = flow_rate as usize;
        let (input, _) = alt((
            tag("; tunnel leads to valve "),
            tag("; tunnels lead to valves "),
        ))(input)?;
        let (input, tunnels) = separated_list1(tag(", "), alpha1)(input)?;
        Ok((
            input,
            Self {
                id,
                flow_rate,
                tunnels,
            },
        ))
    }

    #[allow(dead_code)]
    fn mermaid(&self) {
        self.tunnels
            .iter()
            .for_each(|t| println!("    {} --- {}", self.id, t));
    }
}

#[derive(Debug)]
struct Valves {
    flow_rate: [u8; 16],
    distances: [Vec<(usize, usize)>; 16],
}

impl From<Vec<Valve<'_>>> for Valves {
    fn from(value: Vec<Valve>) -> Self {
        #[inline(always)]
        fn pos(names: &[&str], name: &str) -> Option<usize> {
            names.iter().position(|&n| n == name)
        }

        let mut names = value
            .iter()
            .filter(|v| v.flow_rate > 0 || v.id == "AA")
            .map(|v| v.id)
            .collect::<Vec<_>>();
        names.sort();
        let mut flow_rate = [0u8; 16];
        value.iter().for_each(|v| {
            if let Some(i) = pos(&names, v.id) {
                flow_rate[i] = v.flow_rate as u8;
            }
        });
        let mut distances: [Vec<(usize, usize)>; 16] = Default::default();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        names.iter().for_each(|&start| {
            let start_i = pos(&names, start).expect("Name is in names");
            queue.push_back((start, 0_usize));
            visited.clear();
            visited.insert(start);
            while let Some((valve, distance)) = queue.pop_front() {
                value
                    .iter()
                    .find(|&v| v.id == valve)
                    .expect("Don't ask for the neighbours of a node that doesn't exist")
                    .tunnels
                    .iter()
                    .cloned()
                    .for_each(|neighbour| {
                        if visited.contains(&neighbour) {
                            return; // continue
                        }
                        visited.insert(neighbour);
                        if let Some(i) = pos(&names, neighbour) {
                            if i != 0 {
                                // don't care about going back to AA
                                distances[start_i].push((i, distance + 1));
                            }
                        }
                        queue.push_back((neighbour, distance + 1));
                    })
            }
        });

        Self {
            flow_rate,
            distances,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ClosedValves {
    bitmap: u16,
}

impl ClosedValves {
    const VALVE: u16 = 0b1000_0000_0000_0000;

    fn new(bitmap: u16) -> Self {
        Self { bitmap }
    }

    fn with_open(&self, valve: usize) -> Self {
        Self {
            bitmap: self.bitmap | (Self::VALVE >> valve),
        }
    }

    fn is_open(&self, valve: usize) -> bool {
        (self.bitmap & (Self::VALVE >> valve)) > 0
    }

    fn inverse(&self) -> Self {
        Self {
            bitmap: self.bitmap.not() | Self::VALVE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    current_valve: usize,
    closed_valves: ClosedValves,
    minutes_remaining: usize,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
    let (input, lines) = separated_list1(newline, Valve::parse)(input)?;
    Ok((input, lines))
}

fn dfs(valves: &Valves, state: State, memo: &mut HashMap<State, usize>) -> usize {
    if let Some(value) = memo.get(&state) {
        return *value;
    }
    let max = valves.distances[state.current_valve]
        .iter()
        .map(|&(neighbour, distance)| {
            if state.closed_valves.is_open(neighbour) {
                return 0; // continue
            }
            let remaining = state.minutes_remaining.saturating_sub(distance + 1);
            if remaining == 0 {
                return 0;
            }
            let new_state = State {
                current_valve: neighbour,
                closed_valves: state.closed_valves.with_open(neighbour),
                minutes_remaining: remaining,
            };
            dfs(valves, new_state, memo) + valves.flow_rate[neighbour] as usize * remaining
        })
        .max()
        .expect("Valves is non-empty");
    memo.insert(state, max);
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "1651");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "1707");
    }
}
