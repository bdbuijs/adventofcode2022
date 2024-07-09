use std::{collections::HashMap, vec};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space1, u8 as nomu8},
    multi::separated_list1,
    sequence::pair,
    IResult,
};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

pub fn process_part1(input: &str) -> String {
    let (input, blueprints) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let quality = blueprints.iter().map(|b| b.quality_level(24)).sum::<u16>();
    quality.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, blueprints) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut cache = HashMap::new();
    let max = blueprints
        .iter()
        .take(3)
        .map(|b| {
            cache.clear();
            dfs(b, State::new(32), &mut cache) as u64
        })
        .product::<u64>();
    max.to_string()
}

struct Blueprint {
    id: u8,
    costs: [Vec<(usize, u8)>; 4], // (robot_type, cost)
    max_spend: [u16; 3],          // (ore, clay, obsidian)
}

impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Blueprint ")(input)?;
        let (input, id) = nomu8(input)?;
        let (input, _) = tag(": Each ore robot costs ")(input)?;
        let (input, ore_robot_cost) = Self::parse_resource(input)?;
        let (input, _) = tag("Each clay robot costs ")(input)?;
        let (input, clay_robot_cost) = Self::parse_resource(input)?;
        let (input, _) = tag("Each obsidian robot costs ")(input)?;
        let (input, obsidian_robot_cost) = pair(Self::parse_resource, Self::parse_resource)(input)?;
        let (input, _) = tag("Each geode robot costs ")(input)?;
        let (input, geode_robot_cost) = pair(Self::parse_resource, Self::parse_resource)(input)?;
        let costs = [
            vec![(ORE, ore_robot_cost)],
            vec![(ORE, clay_robot_cost)],
            vec![(ORE, obsidian_robot_cost.0), (CLAY, obsidian_robot_cost.1)],
            vec![(ORE, geode_robot_cost.0), (OBSIDIAN, geode_robot_cost.1)],
        ];
        let max_spend = costs
            .iter()
            .flat_map(|v| v.iter())
            .fold([0, 0, 0], |mut acc, &(i, v)| {
                acc[i] = acc[i].max(v as u16);
                acc
            });

        Ok((
            input,
            Self {
                id,
                costs,
                max_spend,
            },
        ))
    }

    fn parse_resource(input: &str) -> IResult<&str, u8> {
        let (input, value) = nomu8(input)?;
        let (input, _) = space1(input)?;
        let (input, _kind) = alt((tag("ore"), tag("clay"), tag("obsidian")))(input)?;
        let (input, _) = alt((tag(". "), tag("."), tag(" and ")))(input)?;
        Ok((input, value))
    }

    fn quality_level(&self, minutes_remaining: u8) -> u16 {
        let state = State::new(minutes_remaining);
        let mut cache = HashMap::new();
        dfs(self, state, &mut cache) * self.id as u16
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    minutes_remaining: u8,
    resources: [u16; 4], // [ore, clay, obsidian, geode]
    robots: [u16; 4],
}

impl State {
    fn new(minutes_remaining: u8) -> Self {
        Self {
            minutes_remaining,
            resources: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
        }
    }

    fn minimise(&mut self, blueprint: &Blueprint) {
        self.resources
            .iter_mut()
            .zip(blueprint.max_spend.iter())
            .for_each(|(resource, &max_resource)| {
                // we can never spend more than this anyway, so we might as well pretend this is the maximum we have
                *resource = (max_resource * self.minutes_remaining as u16).min(*resource)
            });
    }
}

fn dfs(blueprint: &Blueprint, state: State, cache: &mut HashMap<State, u16>) -> u16 {
    if state.minutes_remaining == 0 {
        return state.resources[GEODE];
    }
    if let Some(result) = cache.get(&state) {
        return *result;
    }

    // best result if we do nothing
    let mut best_result =
        state.resources[GEODE] + state.robots[GEODE] * state.minutes_remaining as u16;

    blueprint.costs.iter().enumerate().for_each(|(typ, costs)| {
        if typ < GEODE && state.robots[typ] >= blueprint.max_spend[typ] {
            return; // continue
        }
        if let Ok(wait) = costs.iter().try_fold(0, |max, &(ctyp, cost)| {
            let cost = cost as u16;
            if state.robots[ctyp] == 0 {
                Err(())
            } else {
                let wait =
                    (cost.saturating_sub(state.resources[ctyp])).div_ceil(state.robots[ctyp]);
                Ok(max.max(wait))
            }
        }) {
            let time = wait + 1;
            if let Some(remaining) = state.minutes_remaining.checked_sub(time as u8) {
                if remaining == 0 {
                    return; // continue
                }
                let mut robots = state.robots;
                let mut resources = state.resources;
                resources.iter_mut().enumerate().for_each(|(typ, amount)| {
                    *amount += robots[typ] * time;
                });
                costs.iter().for_each(|&(ctyp, cost)| {
                    resources[ctyp] -= cost as u16;
                });
                robots[typ] += 1;
                let mut new_state = State {
                    minutes_remaining: remaining,
                    resources,
                    robots,
                };
                new_state.minimise(blueprint);
                best_result = best_result.max(dfs(blueprint, new_state, cache));
            }
        }
    });
    cache.insert(state, best_result);
    best_result
}

fn parse_input(input: &str) -> IResult<&str, Vec<Blueprint>> {
    let (input, lines) = separated_list1(newline, Blueprint::parse)(input)?;
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
        assert_eq!(result, "33");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "3472");
    }
}
