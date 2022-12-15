use std::collections::HashSet;
use std::fs;

fn are_all_different(candidates: &Vec<char>) -> bool {
    let set: HashSet<&char> = candidates.iter().collect();
    return set.len() == candidates.len();
}

fn find_marker(data: &String, marker_len: usize) -> usize {
    let mut chars = data.chars();
    let mut candidates = Vec::new();

    let mut end = 0;
    while end < data.len() {
        if candidates.len() < marker_len {
            let c = chars.next().unwrap();
            candidates.push(c);
            end += 1;
        } else if are_all_different(&candidates) {
            break;
        } else {
            candidates.remove(0);
        }
    }
    end
}

fn main() {
    let path = "input/input.txt";
    let data = fs::read_to_string(path).expect("Error when reading the file");

    let result1 = find_marker(&data, 4);
    let result2 = find_marker(&data, 14);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}
