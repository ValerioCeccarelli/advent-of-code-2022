use std::{collections::HashMap, fs};

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut nodes = parse(&contents);

    let result1 = solution_part1(&mut nodes);
    let result2 = solution_part2(&mut nodes);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

fn solution_part2(nodes: &mut HashMap<String, Node>) -> i64 {
    // humn
    0
}

fn solution_part1(nodes: &mut HashMap<String, Node>) -> i64 {
    let names = nodes.keys().map(|x| x.clone()).collect::<Vec<String>>();

    for name in names {
        let res = calculate(&name, nodes);
        if name == "root" {
            return res;
        }
    }
    0
}

fn calculate(name: &String, nodes: &mut HashMap<String, Node>) -> i64 {
    let node = nodes.get(name).unwrap().clone();
    match node.node_type {
        NodeType::Num(num) => num,
        NodeType::Job(job) => {
            let op1 = calculate(&nodes.get(&job.op1).unwrap().name.clone(), nodes);
            let op2 = calculate(&nodes.get(&job.op2).unwrap().name.clone(), nodes);

            let result = match job.operation {
                Operation::Add => op1 + op2,
                Operation::Sub => op1 - op2,
                Operation::Mul => op1 * op2,
                Operation::Div => op1 / op2,
            };

            nodes.insert(
                node.name.clone(),
                Node {
                    name: node.name.clone(),
                    node_type: NodeType::Num(result),
                },
            );

            result
        }
    }
}

fn parse(input: &String) -> HashMap<String, Node> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let name = split[0][0..4].to_string();
        if split.len() == 2 {
            result.insert(
                name.clone(),
                Node {
                    name: name,
                    node_type: NodeType::Num(split[1].parse().unwrap()),
                },
            );
        } else {
            result.insert(
                name.clone(),
                Node {
                    name: name,
                    node_type: NodeType::Job(Job {
                        op1: split[1].to_string(),
                        op2: split[3].to_string(),
                        operation: match split[2] {
                            "+" => Operation::Add,
                            "-" => Operation::Sub,
                            "*" => Operation::Mul,
                            "/" => Operation::Div,
                            _ => panic!("Unknown operation"),
                        },
                    }),
                },
            );
        }
    }

    result
}

#[derive(Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
enum NodeType {
    Job(Job),
    Num(i64),
}

#[derive(Clone)]
struct Job {
    op1: String,
    op2: String,
    operation: Operation,
}

#[derive(Clone)]
struct Node {
    name: String,
    node_type: NodeType,
}
