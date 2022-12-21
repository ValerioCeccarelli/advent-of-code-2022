use regex::Regex;
use std::{collections::HashMap, mem};

#[derive(Debug)]
pub struct Valve {
    neighbours: Vec<i32>,
    flow: i32,
}

pub fn parse(input: &str) -> (i32, Vec<Valve>) {
    let regex =
        Regex::new(r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let mut valve_aa = 0;

    let mut names = HashMap::<String, i32>::new();
    let lines: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let captures = regex.captures(l).unwrap();
            let i = i as i32;

            let name = captures[1].to_string();

            if name == "AA" {
                valve_aa = i;
            }

            names.insert(name, i);
            let flow = captures[2].parse::<i32>().unwrap();
            let neighbours = captures[3]
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<String>>();

            (flow, neighbours)
        })
        .collect();

    let valves = lines
        .iter()
        .map(|(flow, neighbours)| Valve {
            neighbours: neighbours.iter().map(|n| names[n]).collect(),
            flow: *flow,
        })
        .collect();

    (valve_aa, valves)
}

pub fn solution_part1(start: i32, time: i32, valves: &[Valve]) -> i32 {
    let n = valves.len();
    let mut times_tables = vec![vec![0; n]; n];
    for i in 0..valves.len() {
        let mut visited = vec![false; n];
        let mut to_visit = Vec::new();
        let mut next_to_visit = vec![i as i32];
        let mut l = 0;
        while !next_to_visit.is_empty() {
            mem::swap(&mut to_visit, &mut next_to_visit);
            while let Some(n) = to_visit.pop() {
                if !visited[n as usize] {
                    visited[n as usize] = true;
                    times_tables[i as usize][n as usize] = l;
                    next_to_visit.extend_from_slice(&valves[n as usize].neighbours)
                }
            }
            l += 1;
        }
    }

    let non_broken_valves: Vec<i32> = valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v.flow > 0 { Some(i as i32) } else { None })
        .collect();

    best_score(time, vec![start], &non_broken_valves, &times_tables, valves)
}

fn best_score(
    time_left: i32,
    current_path: Vec<i32>,
    non_broken_valves: &[i32],
    times_tables: &[Vec<i32>],
    valves: &[Valve],
) -> i32 {
    let mut result = Vec::new();

    for v in non_broken_valves {
        if !current_path.contains(v) {
            let time = times_tables[*current_path.last().unwrap() as usize][*v as usize];

            let time_left_after_v = time_left - time - 1;
            if time_left_after_v > 0 {
                let mut path_cloned = current_path.clone();
                path_cloned.push(*v);
                let res = time_left_after_v * valves[*v as usize].flow
                    + best_score(
                        time_left_after_v,
                        path_cloned,
                        non_broken_valves,
                        times_tables,
                        valves,
                    );
                result.push(res);
            } else {
                result.push(0);
            }
        } else {
            result.push(0);
        }
    }

    *result.iter().max().unwrap()
}

fn main() {
    let path = "input/input.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let (start, valves) = parse(&input);

    let result1 = solution_part1(start, 30, &valves);

    println!("Result part 1: {}", result1);
}

// 1673
