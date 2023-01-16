use std::cmp::{max, min};

use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use run_aoc::runner_fn;
use utils::nom_usize;

struct Blueprint {
    id: usize,
    ore_robot_cost_ore: usize,
    clay_robot_cost_ore: usize,
    obs_robot_cost_ore: usize,
    obs_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obs: usize,
    // to prune the search space
    max_ore_needed: usize,
    max_clay_needed: usize,
    max_obs_needed: usize,
}

fn parse_blueprint_id(input: &str) -> IResult<&str, usize> {
    delimited(tag("Blueprint "), nom_usize, tag(":"))(input)
}

fn parse_ore_robot(input: &str) -> IResult<&str, usize> {
    delimited(tag("Each ore robot costs "), nom_usize, tag(" ore."))(input)
}

fn parse_clay_robot(input: &str) -> IResult<&str, usize> {
    delimited(tag("Each clay robot costs "), nom_usize, tag(" ore."))(input)
}

fn parse_obs_robot(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        tuple((
            tag("Each obsidian robot costs "),
            nom_usize,
            tag(" ore and "),
            nom_usize,
            tag(" clay."),
        )),
        |(_, ore, _, clay, _)| (ore, clay),
    )(input)
}

fn parse_geode_robot(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        tuple((
            tag("Each geode robot costs "),
            nom_usize,
            tag(" ore and "),
            nom_usize,
            tag(" obsidian."),
        )),
        |(_, ore, _, obs, _)| (ore, obs),
    )(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            parse_blueprint_id,
            multispace1,
            parse_ore_robot,
            multispace1,
            parse_clay_robot,
            multispace1,
            parse_obs_robot,
            multispace1,
            parse_geode_robot,
        )),
        |(
            id,
            _,
            ore_robot_cost_ore,
            _,
            clay_robot_cost_ore,
            _,
            (obs_robot_cost_ore, obs_robot_cost_clay),
            _,
            (geode_robot_cost_ore, geode_robot_cost_obs),
        )| {
            let ore_costs = vec![
                ore_robot_cost_ore,
                clay_robot_cost_ore,
                obs_robot_cost_ore,
                geode_robot_cost_ore,
            ];
            let max_ore_needed = ore_costs.into_iter().max().unwrap();
            Blueprint {
                id,
                ore_robot_cost_ore,
                clay_robot_cost_ore,
                obs_robot_cost_ore,
                obs_robot_cost_clay,
                geode_robot_cost_ore,
                geode_robot_cost_obs,
                max_ore_needed,
                max_clay_needed: obs_robot_cost_clay,
                max_obs_needed: geode_robot_cost_obs,
            }
        },
    )(input)
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    terminated(separated_list1(many1(newline), parse_blueprint), newline)(input)
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let (leftover, blueprints) = parse_blueprints(input).expect("could not parse input");
    assert_eq!(leftover, "");
    blueprints
}

struct State {
    num_ore_robots: usize,
    ore: usize,
    num_clay_robots: usize,
    clay: usize,
    num_obs_robots: usize,
    obs: usize,
    num_geode_robots: usize,
    geodes: usize,
}

impl State {
    fn new() -> Self {
        State {
            num_ore_robots: 1,
            ore: 0,
            num_clay_robots: 0,
            clay: 0,
            num_obs_robots: 0,
            obs: 0,
            num_geode_robots: 0,
            geodes: 0,
        }
    }

    fn build_ore_robot(&self, bp: &Blueprint, minutes: usize) -> Self {
        State {
            num_ore_robots: self.num_ore_robots + 1,
            ore: self.ore + (self.num_ore_robots * minutes) - bp.ore_robot_cost_ore,
            num_clay_robots: self.num_clay_robots,
            clay: self.clay + (self.num_clay_robots * minutes),
            num_obs_robots: self.num_obs_robots,
            obs: self.obs + (self.num_obs_robots * minutes),
            num_geode_robots: self.num_geode_robots,
            geodes: self.geodes + (self.num_geode_robots * minutes),
        }
    }

    fn build_clay_robot(&self, bp: &Blueprint, minutes: usize) -> Self {
        State {
            num_ore_robots: self.num_ore_robots,
            ore: self.ore + (self.num_ore_robots * minutes) - bp.clay_robot_cost_ore,
            num_clay_robots: self.num_clay_robots + 1,
            clay: self.clay + (self.num_clay_robots * minutes),
            num_obs_robots: self.num_obs_robots,
            obs: self.obs + (self.num_obs_robots * minutes),
            num_geode_robots: self.num_geode_robots,
            geodes: self.geodes + (self.num_geode_robots * minutes),
        }
    }

    fn build_obs_robot(&self, bp: &Blueprint, minutes: usize) -> Self {
        State {
            num_ore_robots: self.num_ore_robots,
            ore: self.ore + (self.num_ore_robots * minutes) - bp.obs_robot_cost_ore,
            num_clay_robots: self.num_clay_robots,
            clay: self.clay + (self.num_clay_robots * minutes) - bp.obs_robot_cost_clay,
            num_obs_robots: self.num_obs_robots + 1,
            obs: self.obs + (self.num_obs_robots * minutes),
            num_geode_robots: self.num_geode_robots,
            geodes: self.geodes + (self.num_geode_robots * minutes),
        }
    }

    fn build_geode_robot(&self, bp: &Blueprint, minutes: usize) -> Self {
        State {
            num_ore_robots: self.num_ore_robots,
            ore: self.ore + (self.num_ore_robots * minutes) - bp.geode_robot_cost_ore,
            num_clay_robots: self.num_clay_robots,
            clay: self.clay + (self.num_clay_robots * minutes),
            num_obs_robots: self.num_obs_robots,
            obs: self.obs + (self.num_obs_robots * minutes) - bp.geode_robot_cost_obs,
            num_geode_robots: self.num_geode_robots + 1,
            geodes: self.geodes + (self.num_geode_robots * minutes),
        }
    }
}

impl Blueprint {
    fn quality(&self) -> usize {
        // max geodes that can be produced in 24 mins
        let g = self.max_geodes(State::new(), 24);
        println!("bp {} max_geodes={}", self.id, g);
        g * self.id
    }

    fn geodes2(&self) -> usize {
        // max geodes that can be produced in 32 mins
        let g = self.max_geodes(State::new(), 32);
        println!("bp {} max_geodes={}", self.id, g);
        g
    }

    fn max_geodes(&self, state: State, mins_left: usize) -> usize {
        // end states and optimizations
        if mins_left == 0 {
            return state.geodes;
        } else if mins_left == 1 {
            // if there's only one minute left, building anything won't help to get more geodes
            return state.geodes + state.num_geode_robots * mins_left;
        } else if mins_left == 2 {
            // with 2 mins left the only way to get more geodes is building a geode bot
            if let Some(mins_to_robot) = self.time_to_geode_robot(&state) {
                if mins_to_robot < mins_left {
                    return self.max_geodes(
                        state.build_geode_robot(&self, mins_to_robot),
                        mins_left - mins_to_robot,
                    );
                }
            }
            return state.geodes + state.num_geode_robots * mins_left;
        }

        // basic idea: branch on which robot to make next
        // (instead of deciding what to do every single minute, similar to day 16)
        let mut branches: Vec<usize> = Vec::new();

        if self.need_more_ore_bots(&state) {
            if let Some(mins_to_robot) = self.time_to_ore_robot(&state) {
                // in order for this one to make sense, there must be time to:
                // - build the ore bot, (mins_to_robot)
                // - get a ore, (1)
                // - build a geode bot, (1)
                // - get a geode (1)
                // so there must be at least 3 mins left for this one
                if mins_left >= mins_to_robot + 3 {
                    branches.push(self.max_geodes(
                        state.build_ore_robot(&self, mins_to_robot),
                        mins_left - mins_to_robot,
                    ));
                }
            }
        }
        if self.need_more_clay_bots(&state) {
            if let Some(mins_to_robot) = self.time_to_clay_robot(&state) {
                // in order for this one to make sense, there must be time to:
                // - build the clay bot, (mins_to_robot)
                // - get a clay, (1)
                // - build obs bot, (1)
                // - get an obs, (1)
                // - build a geode bot, (1)
                // - get a geode (1)
                // so there must be at least 5 mins left for this one
                if mins_left >= mins_to_robot + 5 {
                    branches.push(self.max_geodes(
                        state.build_clay_robot(&self, mins_to_robot),
                        mins_left - mins_to_robot,
                    ));
                }
            }
        }
        if self.need_more_obs_bots(&state) {
            if let Some(mins_to_robot) = self.time_to_obs_robot(&state) {
                // in order for this one to make sense, there must be time to:
                // - build the obs bot, (mins_to_robot)
                // - get an obs, (1)
                // - build a geode bot, (1)
                // - get a geode (1)
                // so there must be at least 3 mins left for this one
                if mins_left >= mins_to_robot + 3 {
                    branches.push(self.max_geodes(
                        state.build_obs_robot(&self, mins_to_robot),
                        mins_left - mins_to_robot,
                    ));
                }
            }
        }
        if let Some(mins_to_robot) = self.time_to_geode_robot(&state) {
            // in order for this one to make sense, there must be time to:
            // - build a geode bot, (mins_to_robot)
            // - get a geode (1)
            // so there must be at least 1 min left for this one
            if mins_left >= mins_to_robot + 1 {
                branches.push(self.max_geodes(
                    state.build_geode_robot(&self, mins_to_robot),
                    mins_left - mins_to_robot,
                ));
            }
        }

        if branches.len() == 0 {
            state.geodes + state.num_geode_robots * mins_left
        } else {
            branches.into_iter().max().unwrap()
        }
    }

    // can only build 1 robot/minute,
    // so more minerals than the max needed for any robot is a waste
    fn need_more_ore_bots(&self, state: &State) -> bool {
        state.num_ore_robots < self.max_ore_needed
    }
    fn need_more_clay_bots(&self, state: &State) -> bool {
        state.num_clay_robots < self.max_clay_needed
    }
    fn need_more_obs_bots(&self, state: &State) -> bool {
        state.num_obs_robots < self.max_obs_needed
    }

    fn time_to_ore_robot(&self, state: &State) -> Option<usize> {
        // (we always have at least one ore robot)
        if state.ore >= self.ore_robot_cost_ore {
            Some(1)
        } else {
            Some(
                ((self.ore_robot_cost_ore - state.ore) as f32 / state.num_ore_robots as f32).ceil()
                    as usize
                    + 1,
            )
        }
    }
    fn time_to_clay_robot(&self, state: &State) -> Option<usize> {
        // (we always have at least one ore robot)
        if state.ore >= self.clay_robot_cost_ore {
            Some(1)
        } else {
            Some(
                ((self.clay_robot_cost_ore - state.ore) as f32 / state.num_ore_robots as f32).ceil()
                    as usize
                    + 1,
            )
        }
    }
    fn time_to_obs_robot(&self, state: &State) -> Option<usize> {
        // (we always have at least one ore robot)
        if state.num_clay_robots == 0 {
            return None;
        }
        let ore_needed = self.obs_robot_cost_ore as isize - state.ore as isize;
        let clay_needed = self.obs_robot_cost_clay as isize - state.clay as isize;

        if ore_needed <= 0 && clay_needed <= 0 {
            Some(1)
        } else {
            Some(
                max(
                    (ore_needed as f32 / state.num_ore_robots as f32).ceil() as usize,
                    (clay_needed as f32 / state.num_clay_robots as f32).ceil() as usize,
                ) + 1,
            )
        }
    }
    fn time_to_geode_robot(&self, state: &State) -> Option<usize> {
        // (we always have at least one ore robot)
        if state.num_obs_robots == 0 {
            return None;
        }
        let ore_needed = self.geode_robot_cost_ore as isize - state.ore as isize;
        let obs_needed = self.geode_robot_cost_obs as isize - state.obs as isize;

        if ore_needed <= 0 && obs_needed <= 0 {
            Some(1)
        } else {
            Some(
                max(
                    (ore_needed as f32 / state.num_ore_robots as f32).ceil() as usize,
                    (obs_needed as f32 / state.num_obs_robots as f32).ceil() as usize,
                ) + 1,
            )
        }
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    let blueprints = parse_input(&file_contents);
    blueprints.iter().map(|b| b.quality()).sum()
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let blueprints = parse_input(&file_contents);
    blueprints[0..min(blueprints.len(), 3)]
        .iter()
        .map(|b| b.geodes2())
        .product()
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day19, part1, example, 33);
    test_fn!(day19, part1, input, 1480);

    test_fn!(day19, part2_SLOW, example, 3472);
    test_fn!(day19, part2_SLOW, input, 3168);
}
