use num::integer::lcm;
use std::fs;

enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

struct Monkey {
    id: usize,
    current_items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    true_dest: usize,
    false_dest: usize,
    items_inspected: u64,
}

fn parse_input(content: &String) -> Vec<Monkey> {
    let monkeys: Vec<Monkey> = content
        .split("\n\n")
        .map(|m_data| str_to_monkey(m_data))
        .collect();
    monkeys
}

fn str_to_monkey(s: &str) -> Monkey {
    let mut lines_it = s.lines();

    let mut id_data = &lines_it.next().unwrap()[7..];
    id_data = &id_data[..id_data.len() - 1];
    let id: usize = id_data.parse().unwrap();

    let current_items_data = &lines_it.next().unwrap()[18..];
    let current_items: Vec<u64> = current_items_data
        .split(", ")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut operation_it = (&lines_it.next().unwrap()[19..]).split(" ");
    _ = operation_it.next().unwrap();
    let op = operation_it.next().unwrap();
    let operand = operation_it.next().unwrap();
    let operation = match op {
        "+" => match operand {
            "old" => Operation::Mult(2),
            s => Operation::Add(s.parse().unwrap()),
        },
        "*" => match operand {
            "old" => Operation::Square,
            s => Operation::Mult(s.parse().unwrap()),
        },
        _ => panic!("Invalid operation"),
    };

    let test_divisor: u64 = (&lines_it.next().unwrap()[21..]).parse().unwrap();
    let true_dest: usize = (&lines_it.next().unwrap()[29..]).parse().unwrap();
    let false_dest: usize = (&lines_it.next().unwrap()[30..]).parse().unwrap();

    Monkey {
        id,
        current_items,
        operation,
        test_divisor,
        true_dest,
        false_dest,
        items_inspected: 0,
    }
}

fn simulate_monkey_business(
    monkeys: &mut Vec<Monkey>,
    rounds: u32,
    worry_control: impl Fn(u64) -> u64,
) {
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            assert!(monkeys[monkey_idx].id == monkey_idx);

            for item_idx in 0..monkeys[monkey_idx].current_items.len() {
                monkeys[monkey_idx].items_inspected += 1;

                let mut item = monkeys[monkey_idx].current_items[item_idx];

                item = match monkeys[monkey_idx].operation {
                    Operation::Add(c) => item + c,
                    Operation::Mult(c) => item * c,
                    Operation::Square => item * item,
                };

                item = worry_control(item);

                let throw_to = if item % monkeys[monkey_idx].test_divisor == 0 {
                    monkeys[monkey_idx].true_dest
                } else {
                    monkeys[monkey_idx].false_dest
                };

                monkeys[throw_to].current_items.push(item);
            }
            monkeys[monkey_idx].current_items.clear();
        }
    }
}

pub fn solution_part1(input: &String) -> u64 {
    let mut monkeys = parse_input(&input);

    simulate_monkey_business(&mut monkeys, 20, |w| w / 3);

    let mut items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected.sort();
    items_inspected[items_inspected.len() - 2] * items_inspected[items_inspected.len() - 1]
}

pub fn solution_part2(input: &String) -> u64 {
    let mut monkeys = parse_input(&input);

    let least_common_multiple = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .fold(1, |accum, elem| lcm(accum, elem));

    simulate_monkey_business(&mut monkeys, 10000, |w| w % least_common_multiple);

    let mut items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected.sort();
    items_inspected[items_inspected.len() - 2] * items_inspected[items_inspected.len() - 1]
}

fn main() {
    let path = "input/input.txt";
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    let result1 = solution_part1(&content);
    let result2 = solution_part2(&content);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}
