use std::fs;

fn main() {
    let path = "input/input.txt";
    let data = fs::read_to_string(path).expect("Error when reading the file");

    let mut results = Vec::<i32>::new();

    let mut acc = 0;
    for s in data.lines() {
        if s != "" {
            let _num: i32 = s.parse().unwrap();
            acc += _num;
        } else {
            results.push(acc);
            acc = 0;
        }
    }
    results.push(acc);

    results.sort();
    results.reverse();

    println!("{} {}", results[0], results[0] + results[1] + results[2]);
}
