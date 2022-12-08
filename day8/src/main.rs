use std::{fs, ops::Range};

fn look_y(board: &Vec<Vec<u32>>, x: usize, y: usize, ys: Range<usize>) -> bool {
    for i in ys {
        if board[i][x] >= board[y][x] {
            return false;
        }
    }
    return true;
}

fn look_x(board: &Vec<Vec<u32>>, x: usize, y: usize, xs: Range<usize>) -> bool {
    for i in xs {
        if board[y][i] >= board[y][x] {
            return false;
        }
    }
    return true;
}

fn solution_part1(board: &Vec<Vec<u32>>) -> u32 {
    let mut count_visible = 0;
    let len = board.len();
    for i in 0..len {
        for j in 0..len {
            let visible1 = look_y(&board, j, i, 0..i);
            let visible2 = look_y(&board, j, i, i + 1..len);
            let visible3 = look_x(&board, j, i, 0..j);
            let visible4 = look_x(&board, j, i, j + 1..len);

            if visible1 || visible2 || visible3 || visible4 {
                count_visible += 1;
            }
        }
    }
    count_visible
}

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let board = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solution1 = solution_part1(&board);
    let solution2 = solution_part2(&board);

    println!("Result part 1: {}", solution1);
    println!("Result part 2: {}", solution2);
}

fn count_y_up(board: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for i in (0..y).rev() {
        count += 1;
        if board[i][x] >= board[y][x] {
            break;
        }
    }
    count
}

fn count_y_down(board: &Vec<Vec<u32>>, x: usize, y: usize, y_max: usize) -> u32 {
    let mut count = 0;
    for i in y + 1..y_max {
        count += 1;
        if board[i][x] >= board[y][x] {
            break;
        }
    }
    count
}

fn count_x_left(board: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for i in (0..x).rev() {
        count += 1;
        if board[y][i] >= board[y][x] {
            break;
        }
    }
    count
}

fn count_x_right(board: &Vec<Vec<u32>>, x: usize, y: usize, x_max: usize) -> u32 {
    let mut count = 0;
    for i in x + 1..x_max {
        count += 1;
        if board[y][i] >= board[y][x] {
            break;
        }
    }
    count
}

fn solution_part2(board: &Vec<Vec<u32>>) -> u32 {
    let mut max_score = 0;
    let len = board.len();
    for i in 0..len {
        for j in 0..len {
            let count1 = count_y_up(&board, j, i);
            let count2 = count_y_down(&board, j, i, len);
            let count3 = count_x_left(&board, j, i);
            let count4 = count_x_right(&board, j, i, len);

            let score = count1 * count2 * count3 * count4;
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}
