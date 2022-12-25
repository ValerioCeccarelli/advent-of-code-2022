use std::fs;

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let values: Vec<i128> = contents.lines().map(|s| s.parse().unwrap()).collect();

    let result1 = solution_part1(&values);
    let result2 = solution_part2(&values);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

fn init_vectors(init_values: &Vec<i128>) -> (Vec<i128>, Vec<i128>) {
    let mut next = Vec::new();
    next.reserve(init_values.len());
    let mut prev = Vec::new();
    prev.reserve(init_values.len());

    for i in 0..init_values.len() {
        let i = i as i128;
        next.push(i + 1);
        prev.push(i - 1);
    }
    prev[0] = prev.len() as i128 - 1;
    next[prev.len() - 1] = 0;

    (next, prev)
}

fn sort(next: &mut Vec<i128>, prev: &mut Vec<i128>, start: &mut i128, values: &Vec<i128>) {
    for i in 0..values.len() {
        let mut delta = values[i];

        let mut current = i as i128;
        if i == *start as usize && values[i] != 0 {
            *start = next[*start as usize];
        }
        if delta > 0 {
            delta = delta % (values.len() as i128 - 1);
            next[prev[i] as usize] = next[i];
            prev[next[i] as usize] = prev[i];

            while delta != 0 {
                current = next[current as usize];
                delta -= 1;
            }
            next[i] = next[current as usize];
            prev[next[current as usize] as usize] = i as i128;
            prev[i] = current;
            next[current as usize] = i as i128;
        } else if delta < 0 {
            delta = -(delta.abs() % (values.len() as i128 - 1));
            next[prev[i] as usize] = next[i];
            prev[next[i] as usize] = prev[i];

            while delta != 0 {
                current = prev[current as usize];
                delta += 1;
            }

            next[i] = current;
            next[prev[current as usize] as usize] = i as i128;
            prev[i] = prev[current as usize];
            prev[current as usize] = i as i128;
        }
    }
}

fn find_result(values: &Vec<i128>, next: &Vec<i128>, start: i128) -> i128 {
    let len = values.len();
    let mut temp = Vec::new();
    temp.reserve(len);
    let mut index = start as usize;

    for _ in values {
        temp.push(values[index]);
        index = next[index] as usize;
    }

    let mut result = 0;
    let index_0 = temp.iter().position(|&r| r == 0).unwrap();

    index = (1000 as usize % len + index_0) % len;
    result += temp[index];

    index = (2000 as usize % len + index_0) % len;
    result += temp[index];

    index = (3000 as usize % len + index_0) % len;
    result += temp[index];

    result
}

fn solution_part2(values: &Vec<i128>) -> i128 {
    let values: Vec<i128> = values.iter().map(|x| x * 811589153 as i128).collect();

    let (mut next, mut prev) = init_vectors(&values);

    let mut start: i128 = 0;
    for _ in 0..10 {
        sort(&mut next, &mut prev, &mut start, &values);
    }

    find_result(&values, &next, start)
}

fn solution_part1(values: &Vec<i128>) -> i128 {
    let (mut next, mut prev) = init_vectors(&values);

    let mut start: i128 = 0;
    sort(&mut next, &mut prev, &mut start, values);

    find_result(&values, &next, start)
}
