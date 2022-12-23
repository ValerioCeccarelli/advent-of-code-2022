use std::{collections::HashSet, fs};

fn parse_input(input: &String) -> (HashSet<(i32, i32, i32)>, (i32, i32, i32)) {
    let mut cubes = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for line in input.lines() {
        let mut line = line.split(",");
        let x = line.next().unwrap().parse().unwrap();
        let y = line.next().unwrap().parse().unwrap();
        let z = line.next().unwrap().parse().unwrap();
        cubes.insert((x, y, z));
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
    }
    (cubes, (max_x, max_y, max_z))
}

fn get_neighbours(cube: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut neighbours = Vec::new();
    neighbours.push((cube.0 - 1, cube.1, cube.2));
    neighbours.push((cube.0 + 1, cube.1, cube.2));
    neighbours.push((cube.0, cube.1 - 1, cube.2));
    neighbours.push((cube.0, cube.1 + 1, cube.2));
    neighbours.push((cube.0, cube.1, cube.2 - 1));
    neighbours.push((cube.0, cube.1, cube.2 + 1));
    neighbours
}

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let (cubes, max_bound) = parse_input(&contents);

    let result1 = solution_part1(&cubes);
    let result2 = solution_part2(&cubes, max_bound);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

fn solution_part1(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut result = 0;

    for cube in cubes {
        let neighbours = get_neighbours(&cube)
            .iter()
            .filter(|neighbour| cubes.contains(neighbour))
            .count() as i32;
        let free_side = 6 - neighbours;
        result += free_side;
    }

    result
}

//dfs find if cube is in a free block of air
fn is_open_air(
    cube: &(i32, i32, i32),
    cubes: &HashSet<(i32, i32, i32)>,
    free_space: &mut HashSet<(i32, i32, i32)>,
    visited: &mut HashSet<(i32, i32, i32)>,
    max_bound: &(i32, i32, i32),
) -> bool {
    if cube.0 <= 0
        || cube.0 >= max_bound.0
        || cube.1 <= 0
        || cube.1 >= max_bound.1
        || cube.2 <= 0
        || cube.2 >= max_bound.2
    {
        return true;
    }
    if visited.contains(cube) {
        return false;
    }
    visited.insert(*cube);
    if free_space.contains(cube) {
        return true;
    }

    for neighbour in get_neighbours(cube) {
        if !cubes.contains(&neighbour) {
            let res = is_open_air(&neighbour, &cubes, free_space, visited, &max_bound);
            if res {
                free_space.insert(*cube);
                return true;
            }
        }
    }

    false
}

fn solution_part2(cubes: &HashSet<(i32, i32, i32)>, max_bound: (i32, i32, i32)) -> i32 {
    let mut result = 0;

    let mut free_space = HashSet::new();

    for cube in cubes {
        let mut neighbours = 0;

        for neighbour in get_neighbours(&cube) {
            if cubes.contains(&neighbour)
                || !is_open_air(
                    &neighbour,
                    &cubes,
                    &mut free_space,
                    &mut HashSet::new(),
                    &max_bound,
                )
            {
                neighbours += 1;
            }
        }

        let free_side = 6 - neighbours;
        result += free_side;
    }

    result
}
