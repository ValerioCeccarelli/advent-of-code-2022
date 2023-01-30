use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse(contents: &str) -> HashSet<(i32, i32)> {
    let mut state = contents
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(y, _)| (x as i32, y as i32))
        })
        .collect::<HashSet<_>>();
    state.reserve(5000);
    state
}

fn solve(state: &mut HashSet<(i32, i32)>) -> (usize, usize) {
    let (mut result1, mut result2) = (0, 0);
    let mut proposals = HashMap::<_, Vec<_>>::with_capacity(10000);
    for t in 0.. {
        for &(x, y) in state.iter() {
            let ns = [
                state.contains(&(x - 1, y - 1)),
                state.contains(&(x - 1, y)),
                state.contains(&(x - 1, y + 1)),
                state.contains(&(x, y - 1)),
                state.contains(&(x, y + 1)),
                state.contains(&(x + 1, y - 1)),
                state.contains(&(x + 1, y)),
                state.contains(&(x + 1, y + 1)),
            ];
            if ns.iter().all(|b| !b) {
                continue;
            }
            let props = [
                (!ns[0] && !ns[1] && !ns[2], (x - 1, y)),
                (!ns[5] && !ns[6] && !ns[7], (x + 1, y)),
                (!ns[0] && !ns[3] && !ns[5], (x, y - 1)),
                (!ns[2] && !ns[4] && !ns[7], (x, y + 1)),
            ];
            for i in 0..4 {
                let (free, pos) = props[(t + i) % 4];
                if free {
                    proposals.entry(pos).or_default().push((x, y));
                    break;
                }
            }
        }
        let mut moved = false;
        for (pos, props) in proposals.drain() {
            if props.len() == 1 {
                moved = true;
                state.remove(&props[0]);
                state.insert(pos);
            }
        }
        if !moved {
            result2 = t + 1;
            break;
        }
        if t == 9 {
            let &minx = state.iter().map(|(x, _)| x).min().unwrap();
            let &maxx = state.iter().map(|(x, _)| x).max().unwrap();
            let &miny = state.iter().map(|(_, y)| y).min().unwrap();
            let &maxy = state.iter().map(|(_, y)| y).max().unwrap();

            let mut count = 0;
            for x in minx..=maxx {
                for y in miny..=maxy {
                    if !state.contains(&(x, y)) {
                        count += 1;
                    }
                }
            }
            result1 = count;
        }
    }
    (result1, result2)
}

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut state = parse(&contents);
    let (result1, result2) = solve(&mut state);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

//4146 957
