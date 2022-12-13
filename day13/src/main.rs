use std::{fs, iter::Peekable, str::Chars};

#[derive(PartialEq, Debug)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

#[derive(PartialEq, Debug)]
enum Result {
    Ordered,
    Unordered,
    None,
}

fn parse(chars: &mut Peekable<Chars>) -> Vec<Packet> {
    let mut result = Vec::new();
    // if *next != '[' {
    //     panic!("Expected '[' but got <{}>", *next);
    // }
    let mut first_time = true;
    loop {
        let next = chars.peek().unwrap();
        if first_time && *next == '[' {
            chars.next();
            first_time = false;
            continue;
        }
        if *next == ']' {
            chars.next();
            break;
        }
        if *next == ',' {
            chars.next();
            continue;
        }
        if *next == '[' {
            let packets = parse(chars);
            result.push(Packet::List(packets));
        } else {
            let mut num: u32 = next.to_string().parse().unwrap();
            chars.next();
            let other = chars.peek().unwrap();
            if *other == '0' {
                num *= 10;
                chars.next();
            }
            result.push(Packet::Num(num));
        }
    }
    result
}

fn parse_line(line: &str) -> Vec<Packet> {
    let mut chars = line.chars().peekable();
    let result = parse(&mut chars);
    result
}

fn compare_packet(first: &Vec<Packet>, second: &Vec<Packet>) -> Result {
    let mut first = first.iter();
    let mut second = second.iter();

    loop {
        let first = first.next();
        let second = second.next();
        if first.is_none() && second.is_none() {
            return Result::None;
        }
        if first.is_none() && second.is_some() {
            return Result::Ordered;
        }
        if first.is_some() && second.is_none() {
            return Result::Unordered;
        }
        // Both are Some
        let first = first.unwrap();
        let second = second.unwrap();

        match (first, second) {
            (Packet::Num(a), Packet::Num(b)) => {
                if a < b {
                    return Result::Ordered;
                } else if a > b {
                    return Result::Unordered;
                }
            }
            (Packet::List(a), Packet::List(b)) => {
                let result = compare_packet(a, b);
                if result == Result::Unordered || result == Result::Ordered {
                    return result;
                }
            }
            (Packet::Num(a), Packet::List(b)) => {
                let result = compare_packet(&mut vec![Packet::Num(*a)], b);
                if result == Result::Unordered || result == Result::Ordered {
                    return result;
                }
            }
            (Packet::List(b), Packet::Num(a)) => {
                let result = compare_packet(b, &mut vec![Packet::Num(*a)]);
                if result == Result::Unordered || result == Result::Ordered {
                    return result;
                }
            }
        }
    }
}

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let result1 = result_part1(&contents);
    let result2 = result_part2(&contents);

    println!("Result: {}", result1);
    println!("Result: {}", result2);
}

fn result_part2(contents: &String) -> i32 {
    let mut packets = Vec::new();
    for pair in contents.split("\n\n") {
        let mut pair = pair.split("\n");
        let first = pair.next().unwrap();
        let second = pair.next().unwrap();

        let first = parse_line(first);
        let second = parse_line(second);

        packets.push(first);
        packets.push(second);
    }

    let dec_key1 = parse_line("[[2]]");
    let dec_key2 = parse_line("[[6]]");

    packets.push(dec_key1);
    packets.push(dec_key2);

    packets.sort_by(|a, b| {
        let result = compare_packet(a, b);
        match result {
            Result::Ordered => std::cmp::Ordering::Less,
            Result::Unordered => std::cmp::Ordering::Greater,
            Result::None => std::cmp::Ordering::Equal,
        }
    });

    let mut result = 1;
    for (index, packet) in packets.iter().enumerate() {
        if packet == &parse_line("[[2]]") || packet == &parse_line("[[6]]") {
            result *= index as i32 + 1;
        }
    }
    result
}

fn result_part1(contents: &String) -> i32 {
    let mut index = 1;
    let mut result = 0;
    for pair in contents.split("\n\n") {
        let mut pair = pair.split("\n");
        let first = pair.next().unwrap();
        let second = pair.next().unwrap();

        let first = parse_line(first);
        let second = parse_line(second);

        let res = compare_packet(&first, &second);
        if res == Result::Ordered {
            result += index;
        }

        index += 1;
    }
    result
}
