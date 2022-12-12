use std::collections::VecDeque;
use std::fs;

#[derive(Clone)]
struct Input {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    visited: Vec<Vec<bool>>,
    distance: Vec<Vec<u32>>,
}

fn parse_input(path: &str) -> Input {
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut distance: Vec<Vec<u32>> = Vec::new();
    for line in content.lines() {
        let mut row: Vec<u8> = Vec::new();
        let mut row_visited: Vec<bool> = Vec::new();
        let mut row_distance: Vec<u32> = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                start = (row.len(), map.len());
                row.push('a' as u8);
            } else if c == 'E' {
                end = (row.len(), map.len());
                row.push('z' as u8);
            } else {
                row.push(c as u8);
            }
            row_visited.push(false);
            row_distance.push(0);
        }
        map.push(row);
        visited.push(row_visited);
        distance.push(row_distance);
    }

    Input {
        map,
        start,
        end,
        visited,
        distance,
    }
}

/// Generate four directions from a point;
/// maxx and maxy are the exclusive max values
fn generate_four_directions(x: usize, y: usize, maxx: usize, maxy: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if x < maxx - 1 {
        result.push((x + 1, y));
    }
    if y < maxy - 1 {
        result.push((x, y + 1));
    }
    result
}

fn main() {
    let path = "input/input.txt";
    let mut input = parse_input(path);

    let result1 = solution_part1(&mut input.clone());
    let result2 = solution_part2(&mut input);

    println!("Result part 1 {}", result1);
    println!("Result part 2 {}", result2);
}

fn solution_part2(input: &mut Input) -> u32 {
    let mut min_path = u32::max_value();
    for y in 0..input.map.len() {
        for x in 0..input.map[0].len() {
            if input.map[y][x] == 'a' as u8 {
                let mut new_input = input.clone();
                new_input.start = (x, y);
                let result = solution_part1(&mut new_input);

                if result != 0 {
                    min_path = min_path.min(result);
                }
            }
        }
    }
    min_path
}

fn solution_part1(input: &mut Input) -> u32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(input.start);

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let distance = input.distance[y][x];
        if input.visited[y][x] {
            continue;
        }
        input.visited[y][x] = true;

        let directions = generate_four_directions(x, y, input.map[0].len(), input.map.len());
        for direction in directions {
            let (nx, ny) = direction;
            if input.visited[ny][nx] {
                continue;
            }

            if input.map[ny][nx] <= input.map[y][x] + 1 {
                if input.distance[ny][nx] < distance + 1 {
                    input.distance[ny][nx] = distance + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    input.distance[input.end.1][input.end.0]
}
