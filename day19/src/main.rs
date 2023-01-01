use regex::Regex;
use std::{collections::HashMap, fs};

fn dfs(state: &State, memo: &mut HashMap<State, i32>, blueprint: &Blueprint, maxes: &Maxes) -> i32 {
    if state.time_left == 1 {
        return state.material.geode + state.geode_bot;
    }
    if let Some(v) = memo.get(&state) {
        return *v;
    }

    let mut ans = state.material.geode;

    //geode bot creation
    {
        let mut new_state = state.clone();
        new_state.time_left -= 1;

        new_state.material = new_state.material - blueprint.geode_bot;

        if new_state.material.is_valid()
            && blueprint.max_ore() >= new_state.ore_bot
            && blueprint.max_clay() >= new_state.clay_bot
            && blueprint.max_obsidian() >= new_state.obsidian_bot
        {
            new_state.material.ore += new_state.ore_bot;
            new_state.material.clay += new_state.clay_bot;
            new_state.material.obsidian += new_state.obsidian_bot;
            new_state.material.geode += new_state.geode_bot;

            new_state.material.ore =
                std::cmp::min(new_state.material.ore, maxes.ore * new_state.time_left);
            new_state.material.clay =
                std::cmp::min(new_state.material.clay, maxes.clay * new_state.time_left);
            new_state.material.obsidian = std::cmp::min(
                new_state.material.obsidian,
                maxes.obsidian * new_state.time_left,
            );

            new_state.geode_bot += 1;

            ans = std::cmp::max(ans, dfs(&new_state, memo, blueprint, maxes));

            memo.insert(*state, ans);
            return ans;
        }
    }
    //obsidian bot creation
    {
        let mut new_state = state.clone();
        new_state.time_left -= 1;

        new_state.material = new_state.material - blueprint.obsidian_bot;

        if new_state.material.is_valid()
            && blueprint.max_ore() >= new_state.ore_bot
            && blueprint.max_clay() >= new_state.clay_bot
            && blueprint.max_obsidian() >= new_state.obsidian_bot
        {
            new_state.material.ore += new_state.ore_bot;
            new_state.material.clay += new_state.clay_bot;
            new_state.material.obsidian += new_state.obsidian_bot;
            new_state.material.geode += new_state.geode_bot;

            new_state.material.ore =
                std::cmp::min(new_state.material.ore, maxes.ore * new_state.time_left);
            new_state.material.clay =
                std::cmp::min(new_state.material.clay, maxes.clay * new_state.time_left);
            new_state.material.obsidian = std::cmp::min(
                new_state.material.obsidian,
                maxes.obsidian * new_state.time_left,
            );

            new_state.obsidian_bot += 1;

            ans = std::cmp::max(ans, dfs(&new_state, memo, blueprint, maxes));
        }
    }
    //clay bot creation
    {
        let mut new_state = state.clone();
        new_state.time_left -= 1;

        new_state.material = new_state.material - blueprint.clay_bot;

        if new_state.material.is_valid()
            && blueprint.max_ore() >= new_state.ore_bot
            && blueprint.max_clay() >= new_state.clay_bot
            && blueprint.max_obsidian() >= new_state.obsidian_bot
        {
            new_state.material.ore += new_state.ore_bot;
            new_state.material.clay += new_state.clay_bot;
            new_state.material.obsidian += new_state.obsidian_bot;
            new_state.material.geode += new_state.geode_bot;

            new_state.material.ore =
                std::cmp::min(new_state.material.ore, maxes.ore * new_state.time_left);
            new_state.material.clay =
                std::cmp::min(new_state.material.clay, maxes.clay * new_state.time_left);
            new_state.material.obsidian = std::cmp::min(
                new_state.material.obsidian,
                maxes.obsidian * new_state.time_left,
            );

            new_state.clay_bot += 1;

            ans = std::cmp::max(ans, dfs(&new_state, memo, blueprint, maxes));
        }
    }
    //ore bot creation
    {
        let mut new_state = state.clone();
        new_state.time_left -= 1;

        new_state.material = new_state.material - blueprint.ore_bot;

        if new_state.material.is_valid()
            && blueprint.max_ore() >= new_state.ore_bot
            && blueprint.max_clay() >= new_state.clay_bot
            && blueprint.max_obsidian() >= new_state.obsidian_bot
        {
            new_state.material.ore += new_state.ore_bot;
            new_state.material.clay += new_state.clay_bot;
            new_state.material.obsidian += new_state.obsidian_bot;
            new_state.material.geode += new_state.geode_bot;

            new_state.material.ore =
                std::cmp::min(new_state.material.ore, maxes.ore * new_state.time_left);
            new_state.material.clay =
                std::cmp::min(new_state.material.clay, maxes.clay * new_state.time_left);
            new_state.material.obsidian = std::cmp::min(
                new_state.material.obsidian,
                maxes.obsidian * new_state.time_left,
            );

            new_state.ore_bot += 1;

            ans = std::cmp::max(ans, dfs(&new_state, memo, blueprint, maxes));
        }
    }
    //no bot creation
    {
        let mut new_state = state.clone();
        new_state.time_left -= 1;

        new_state.material.ore += new_state.ore_bot;
        new_state.material.clay += new_state.clay_bot;
        new_state.material.obsidian += new_state.obsidian_bot;
        new_state.material.geode += new_state.geode_bot;

        new_state.material.ore =
            std::cmp::min(new_state.material.ore, maxes.ore * new_state.time_left);
        new_state.material.clay =
            std::cmp::min(new_state.material.clay, maxes.clay * new_state.time_left);
        new_state.material.obsidian = std::cmp::min(
            new_state.material.obsidian,
            maxes.obsidian * new_state.time_left,
        );

        ans = std::cmp::max(ans, dfs(&new_state, memo, blueprint, maxes));
    }

    memo.insert(*state, ans);
    return ans;
}

fn main() {
    let path = "input/input.txt";
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    let blueprints = parse(&content);

    let mut result1 = 0;
    let mut result2 = 1;
    for (index, blueprint) in blueprints.iter().enumerate() {
        let mut memo = HashMap::new();
        let maxes = blueprint.get_maxes();

        result1 += blueprint.id * dfs(&State::new(24), &mut memo, &blueprint, &maxes);
        if index < 3 {
            result2 *= dfs(&State::new(32), &mut memo, &blueprint, &maxes);
        }
    }
    println!("Solution part 1: {}", result1); // 1264
    println!("Solution part 2: {}", result2); // 13475
}

fn parse(input: &String) -> Vec<Blueprint> {
    let re = Regex::new(r"\d+").unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        let nums = re
            .captures_iter(line)
            .map(|x| x[0].parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let blueprint = Blueprint {
            id: nums[0],
            ore_bot: Material {
                ore: nums[1],
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_bot: Material {
                ore: nums[2],
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_bot: Material {
                ore: nums[3],
                clay: nums[4],
                obsidian: 0,
                geode: 0,
            },
            geode_bot: Material {
                ore: nums[5],
                clay: 0,
                obsidian: nums[6],
                geode: 0,
            },
        };
        result.push(blueprint);
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Material {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Material {
    fn is_valid(&self) -> bool {
        self.ore >= 0 && self.clay >= 0 && self.obsidian >= 0 && self.geode >= 0
    }
}

impl std::ops::Sub for Material {
    type Output = Material;

    fn sub(self, other: Material) -> Material {
        Material {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

struct Blueprint {
    id: i32,
    ore_bot: Material,
    clay_bot: Material,
    obsidian_bot: Material,
    geode_bot: Material,
}

impl Blueprint {
    fn max_ore(&self) -> i32 {
        self.ore_bot
            .ore
            .max(self.clay_bot.ore)
            .max(self.obsidian_bot.ore)
            .max(self.geode_bot.ore)
    }
    fn max_clay(&self) -> i32 {
        //other bots don't use clay
        self.obsidian_bot.clay
    }
    fn max_obsidian(&self) -> i32 {
        //other bots don't use obsidian
        self.geode_bot.obsidian
    }
    fn get_maxes(&self) -> Maxes {
        Maxes {
            ore: self.max_ore(),
            clay: self.max_clay(),
            obsidian: self.max_obsidian(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct State {
    material: Material,
    ore_bot: i32,
    clay_bot: i32,
    obsidian_bot: i32,
    geode_bot: i32,
    time_left: i32,
}

impl State {
    fn new(time_left: i32) -> State {
        State {
            material: Material {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            ore_bot: 1,
            clay_bot: 0,
            obsidian_bot: 0,
            geode_bot: 0,
            time_left,
        }
    }
}

struct Maxes {
    ore: i32,
    clay: i32,
    obsidian: i32,
}
