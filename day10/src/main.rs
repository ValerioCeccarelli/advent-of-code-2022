use std::fs;

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let result1 = result_part1(&contents);
    println!("Result part 1: {}", result1);

    draw_part2(&contents);
}

fn draw_part2(contents: &String) {
    print!("\n\n\n");

    let mut cycle = 0;
    let mut line_result = String::new();
    let mut register = 1;
    for line in contents.lines() {
        if cycle >= 40 {
            cycle -= 40;
            println!("{}", line_result);
            line_result = String::new();
        }

        let mut line = line.split_whitespace();
        let command = line.next().unwrap();

        if command == "noop" {
            if cycle == register - 1 || cycle == register || cycle == register + 1 {
                line_result.push_str("#");
            } else {
                line_result.push_str(".");
            }
            cycle += 1;
        } else if command == "addx" {
            let num: i32 = line.next().unwrap().parse().unwrap();
            if cycle == register - 1 || cycle == register || cycle == register + 1 {
                line_result.push_str("#");
            } else {
                line_result.push_str(".");
            }
            cycle += 1;
            if cycle >= 40 {
                cycle -= 40;
                println!("{}", line_result);
                line_result = String::new();
            }
            if cycle == register - 1 || cycle == register || cycle == register + 1 {
                line_result.push_str("#");
            } else {
                line_result.push_str(".");
            }
            cycle += 1;
            register += num;
        } else {
            panic!("Unknown command: {}", command);
        }
    }
    println!("{}", line_result);
    print!("\n\n\n");
}

fn result_part1(contents: &String) -> i32 {
    let mut cycle = 1;
    let mut register = 1;
    let mut result = 0;
    let cycles_to_compare = vec![20, 60, 100, 140, 180, 220];
    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let command = line.next().unwrap();

        if command == "noop" {
            cycle += 1;
        } else if command == "addx" {
            let num: i32 = line.next().unwrap().parse().unwrap();
            if cycles_to_compare.contains(&cycle) {
                result += register * cycle;
            } else if cycles_to_compare.contains(&(cycle + 1)) {
                result += register * (cycle + 1);
            }
            register += num;
            cycle += 2;
        } else {
            panic!("Unknown command: {}", command);
        }
    }
    result
}

// 16060
// BACEKLHF
