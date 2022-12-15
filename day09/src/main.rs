use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let part1 = solution_part1(&contents);
    let part2 = solution_part2(&contents);

    println!("Solution part 1 {}", part1);
    println!("Solution part 2 {}", part2);
}

fn solution_part2(contents: &String) -> usize {
    let mut nodes = Vec::new();
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for _ in 0..10 {
        nodes.push((0, 0));
    }

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let direction = line.next().unwrap();
        let distance = line.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                "U" => nodes[0].0 -= 1,
                "D" => nodes[0].0 += 1,
                "R" => nodes[0].1 += 1,
                "L" => nodes[0].1 -= 1,
                _ => panic!("Unknown direction"),
            }
            for i in 1..10 {
                let mut distance = 1e9 as i32;
                let mut movement = (0, 0);
                for a in -1..2 {
                    for b in -1..2 {
                        let diff_y = i32::abs(nodes[i].0 + a - nodes[i - 1].0);
                        let diff_x = i32::abs(nodes[i].1 + b - nodes[i - 1].1);

                        if diff_y + diff_x < distance {
                            distance = diff_y + diff_x;
                            movement = (a, b);
                        }
                    }
                }
                if distance == 0 {
                    continue;
                }
                nodes[i].0 += movement.0;
                nodes[i].1 += movement.1;
            }
            visited.insert(nodes[9]);
        }
    }

    visited.len()
}

fn solution_part1(contents: &String) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let direction = line.next().unwrap();
        let distance = line.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                _ => panic!("Unknown direction"),
            }
            let mut distance = 1e9 as i32;
            let mut movement = (0, 0);
            for a in -1..2 {
                for b in -1..2 {
                    let diff_y = i32::abs(tail.0 + a - head.0);
                    let diff_x = i32::abs(tail.1 + b - head.1);

                    if diff_y + diff_x < distance {
                        distance = diff_y + diff_x;
                        movement = (a, b);
                    }
                }
            }
            if distance == 0 {
                continue;
            }
            tail.0 += movement.0;
            tail.1 += movement.1;
            visited.insert(tail);
        }
    }
    visited.len()
}
