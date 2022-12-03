use std::collections::HashSet;
use std::fs;

fn lecter_to_priority(c: char) -> Option<i32> {
    let item = c as i32;
    if 65 <= item && item <= 90 {
        Some(item - 65 + 27)
    } else if 97 <= item && item <= 122 {
        Some(item - 97 + 1)
    } else {
        None
    }
}

fn part1(data: &String) -> i32 {
    let mut priority_sum = 0;
    for line in data.lines() {
        let len = line.len();
        let compartment1 = &line[..len / 2];
        let compartment2 = &line[len / 2..];

        for c in compartment1.chars() {
            if compartment2.contains(c) {
                priority_sum += lecter_to_priority(c).unwrap();
                break;
            }
        }
    }
    priority_sum
}

fn part2(data: &String) -> i32 {
    let mut result = 0;
    let mut lines = data.lines();
    loop {
        let first_opt = lines.next();
        if first_opt.is_none() {
            break;
        }
        let first_line = first_opt.unwrap();
        let second_line: HashSet<char> = lines.next().unwrap().chars().collect();
        let third_line: HashSet<char> = lines.next().unwrap().chars().collect();

        for c_first in first_line.chars() {
            if second_line.contains(&c_first) && third_line.contains(&c_first) {
                result += lecter_to_priority(c_first).unwrap();
                break;
            }
        }
    }
    result
}

fn main() {
    let path = "input/input.txt";
    let data = fs::read_to_string(path).expect("Error when reading the file");

    let result1 = part1(&data);
    let result2 = part2(&data);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}
