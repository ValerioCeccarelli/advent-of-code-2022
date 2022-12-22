use std::collections::HashSet;

fn can_left(parts: &Vec<(u128, u128)>, rocks: &HashSet<(u128, u128)>) -> bool {
    for (x, y) in parts {
        if *x == 0 {
            return false;
        }
        if rocks.contains(&(x - 1, *y)) {
            return false;
        }
    }
    true
}

fn can_right(parts: &Vec<(u128, u128)>, rocks: &HashSet<(u128, u128)>) -> bool {
    for (x, y) in parts {
        if x + 1 > 6 {
            return false;
        }
        if rocks.contains(&(x + 1, *y)) {
            return false;
        }
    }
    true
}

fn can_down(parts: &Vec<(u128, u128)>, rocks: &HashSet<(u128, u128)>) -> bool {
    for (x, y) in parts {
        if *y == 1 {
            return false;
        }
        if rocks.contains(&(*x, y - 1)) {
            return false;
        }
    }
    true
}

fn move_left(parts: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    parts.iter().map(|(x, y)| (x - 1, *y)).collect()
}

fn move_right(parts: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    parts.iter().map(|(x, y)| (x + 1, *y)).collect()
}

fn move_down(parts: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    parts.iter().map(|(x, y)| (*x, y - 1)).collect()
}

fn get_max_y(parts: &Vec<(u128, u128)>) -> u128 {
    *parts.iter().map(|(_, y)| y).max().unwrap()
}

fn create_rocks(start_y: u128, rock_type: u128) -> Vec<(u128, u128)> {
    match rock_type {
        // ####
        0 => vec![(2, start_y), (3, start_y), (4, start_y), (5, start_y)],
        // .#.
        // ###
        // .#.
        1 => vec![
            (3, start_y),
            (2, start_y + 1),
            (3, start_y + 1),
            (4, start_y + 1),
            (3, start_y + 2),
        ],
        // ..#
        // ..#
        // ###
        2 => vec![
            (2, start_y),
            (3, start_y),
            (4, start_y),
            (4, start_y + 1),
            (4, start_y + 2),
        ],
        // #
        // #
        // #
        // #
        3 => vec![
            (2, start_y),
            (2, start_y + 1),
            (2, start_y + 2),
            (2, start_y + 3),
        ],
        // ##
        // ##
        4 => vec![
            (2, start_y),
            (3, start_y),
            (2, start_y + 1),
            (3, start_y + 1),
        ],
        _ => panic!("Invalid rock type"),
    }
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

fn parse_input(contents: &String) -> Vec<Move> {
    contents
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'<' => Move::Left,
            b'>' => Move::Right,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn print_all(rock: &Vec<(u128, u128)>, rocks: &HashSet<(u128, u128)>) {
    println!("---------------------------------");
    let max_y = get_max_y(rock);
    for y in (1..max_y + 1).rev() {
        for x in 0..7 {
            if rock.contains(&(x, y)) {
                print!("@");
            } else if rocks.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("---------------------------------");
}

fn solution(moves: &Vec<Move>, rock_num: u128) -> u128 {
    let mut rocks = HashSet::new();
    let mut max_height = 0;

    let mut moves = moves.iter().cycle();

    for i in 0..rock_num {
        let rock_type = i % 5;
        let mut rock = create_rocks(max_height + 4, rock_type);
        //println!("Rock {:?}", rock);
        loop {
            let m = moves.next().unwrap();
            // print_all(&rock, &rocks);
            // std::io::stdin().read_line(&mut String::new()).unwrap();
            match m {
                Move::Left => {
                    if can_left(&rock, &rocks) {
                        rock = move_left(rock);
                    }
                }
                Move::Right => {
                    if can_right(&rock, &rocks) {
                        rock = move_right(rock);
                    }
                }
            }
            if can_down(&rock, &rocks) {
                rock = move_down(rock);
            } else {
                break;
            }
        }
        rocks.extend(rock.iter());
        max_height = u128::max(max_height, get_max_y(&rock));
        // print_all(&rock, &rocks);
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    max_height
}

fn main() {
    let path = "input/input.txt";
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");

    let moves = parse_input(&contents);

    let result1 = solution(&moves, 2022);
    println!("Solution part 1: {}", result1);

    println!("Solution part 2 takes too long to calculate... (not solved yet)");
    let result2 = solution(&moves, 1000000000000);
    println!("Solution part 2: {}", result2);
}
