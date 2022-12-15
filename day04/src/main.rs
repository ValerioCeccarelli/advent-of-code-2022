use std::fs;

fn main() {
    let path = "input/input.txt";
    let data = fs::read_to_string(path).expect("Error when reading the file");

    let mut count_contained = 0;
    let mut count_overlap = 0;
    for line in data.lines() {
        let mut pair = line.split(",");
        let mut first = pair.next().unwrap().split("-");
        let mut second = pair.next().unwrap().split("-");
        let first_beg: i32 = first.next().unwrap().parse().unwrap();
        let first_end: i32 = first.next().unwrap().parse().unwrap();
        let second_beg: i32 = second.next().unwrap().parse().unwrap();
        let second_end: i32 = second.next().unwrap().parse().unwrap();

        if (first_beg <= second_beg && second_end <= first_end)
            || (second_beg <= first_beg && first_end <= second_end)
        {
            count_contained += 1;
        }
        if !(first_beg > second_end || first_end < second_beg) {
            count_overlap += 1;
        }
    }
    println!("Result part 1 {}", count_contained);
    println!("Result part 2 {}", count_overlap);
}
