use std::fs;

fn snafu_to_decimal(s: &str) -> usize {
    s.chars().fold(0, |n, d| {
        n * 5 + "=-012".chars().position(|x| x == d).unwrap() - 2
    })
}

fn decimal_to_snafu(n: usize) -> String {
    if n == 0 {
        String::new()
    } else {
        decimal_to_snafu((n + 2) / 5) + ["0", "1", "2", "=", "-"][n % 5]
    }
}

fn main() {
    let path = "input/input.txt";
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");

    let sum = input.lines().map(snafu_to_decimal).sum();
    let result = decimal_to_snafu(sum);

    println!("Solution part 1: {}", result);
}
