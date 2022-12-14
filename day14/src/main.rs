use std::{collections::HashSet, fs};

fn parse_input(input: &String) -> (HashSet<(u32, u32)>, u32) {
    let mut result = HashSet::new();
    let mut max_y = 0;
    for rock_path in input.lines() {
        let mut rock_path = rock_path.split(" -> ");
        let mut from = rock_path.next().unwrap().split(",");

        let mut from_x: u32 = from.next().unwrap().parse().unwrap();
        let mut from_y: u32 = from.next().unwrap().parse().unwrap();
        max_y = max_y.max(from_y);

        loop {
            let to = rock_path.next();
            if to.is_none() {
                break;
            }
            let mut to = to.unwrap().split(",");
            let to_x = to.next().unwrap().parse().unwrap();
            let to_y = to.next().unwrap().parse().unwrap();
            if from_x == to_x {
                for y in from_y.min(to_y)..=from_y.max(to_y) {
                    result.insert((from_x, y));
                }
            } else {
                for x in from_x.min(to_x)..=from_x.max(to_x) {
                    result.insert((x, from_y));
                }
            }
            from_x = to_x;
            from_y = to_y;
            max_y = max_y.max(to_y);
        }
    }
    (result, max_y)
}

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let (rocks, max_y) = parse_input(&contents);

    println!("max y: {}", max_y);

    let result1 = solution_part1(rocks.clone(), max_y + 2);
    let result2 = solution_part2(rocks, max_y + 2);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

fn solution_part2(rocks: HashSet<(u32, u32)>, max_y: u32) -> u32 {
    let mut rocks = rocks;
    let generator = (500, 0);
    let mut result = 0;

    loop {
        if rocks.contains(&generator) {
            break;
        }
        let mut sand = (generator.0, generator.1);
        loop {
            if sand.1 + 1 == max_y {
                rocks.insert(sand);
                result += 1;
                break;
            } else if !rocks.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                rocks.insert(sand);
                result += 1;
                break;
            }
        }
    }
    result
}

fn solution_part1(rocks: HashSet<(u32, u32)>, max_y: u32) -> u32 {
    let mut rocks = rocks;
    let generator = (500, 0);
    let mut result = 0;

    loop {
        if rocks.contains(&generator) {
            break;
        }
        let mut sand = (generator.0, generator.1);
        loop {
            if !rocks.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                rocks.insert(sand);
                result += 1;
                break;
            }
            if sand.1 >= max_y {
                return result;
            }
        }
    }
    result
}
