use std::fs;

fn part1(commands: &str, stacks: &mut Vec<String>) -> String {
    for line in commands.lines() {
        let mut words = line.split(" ");
        let quantity: usize = words.nth(1).unwrap().parse().unwrap();
        let from: usize = words.nth(1).unwrap().parse().unwrap();
        let to: usize = words.nth(1).unwrap().parse().unwrap();

        for _ in 0..quantity {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }
    let mut result = String::new();
    for stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
}

fn part2(commands: &str, stacks: &mut Vec<String>) -> String {
    for line in commands.lines() {
        let mut words = line.split(" ");
        let quantity: usize = words.nth(1).unwrap().parse().unwrap();
        let from: usize = words.nth(1).unwrap().parse().unwrap();
        let to: usize = words.nth(1).unwrap().parse().unwrap();

        let mut temp_stack = String::new();
        for _ in 0..quantity {
            let c = stacks[from - 1].pop().unwrap();
            temp_stack.push(c);
        }
        for _ in 0..quantity {
            let c = temp_stack.pop().unwrap();
            stacks[to - 1].push(c);
        }
    }
    let mut result = String::new();
    for stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
}

fn main() {
    let path = "input/input.txt";
    let data = fs::read_to_string(path).expect("Error when reading the file");

    let mut data_split = data.split("\n\n");
    let commands = data_split.nth(1).unwrap();

    let mut stacks1 = create_stacks();
    let mut stacks2 = create_stacks();

    let result1 = part1(commands, &mut stacks1);
    let result2 = part2(commands, &mut stacks2);

    println!("Result part 1 {}", result1);
    println!("Result part 2 {}", result2);
}

fn create_stacks() -> Vec<String> {
    let stacks = vec![
        "GTRW".to_string(),
        "GCHPMSVW".to_string(),
        "CLTSGM".to_string(),
        "JHDMWRF".to_string(),
        "PQLHSWFJ".to_string(),
        "PJDNFMS".to_string(),
        "ZBDFGCSJ".to_string(),
        "RTB".to_string(),
        "HNWLC".to_string(),
    ];
    stacks
}
