use std::fs;

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSOR_SCORE: i32 = 3;

const LOSE_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

fn first_part_score(first: &str, second: &str) -> i32 {
    if first == "A" {
        if second == "X" {
            ROCK_SCORE + DRAW_SCORE
        } else if second == "Y" {
            PAPER_SCORE + WIN_SCORE
        } else {
            SCISSOR_SCORE + LOSE_SCORE
        }
    } else if first == "B" {
        if second == "X" {
            ROCK_SCORE + LOSE_SCORE
        } else if second == "Y" {
            PAPER_SCORE + DRAW_SCORE
        } else {
            SCISSOR_SCORE + WIN_SCORE
        }
    } else {
        if second == "X" {
            ROCK_SCORE + WIN_SCORE
        } else if second == "Y" {
            PAPER_SCORE + LOSE_SCORE
        } else {
            SCISSOR_SCORE + DRAW_SCORE
        }
    }
}

fn second_part_score(first: &str, second: &str) -> i32 {
    if first == "A" {
        if second == "X" {
            SCISSOR_SCORE + LOSE_SCORE
        } else if second == "Y" {
            ROCK_SCORE + DRAW_SCORE
        } else {
            PAPER_SCORE + WIN_SCORE
        }
    } else if first == "B" {
        if second == "X" {
            ROCK_SCORE + LOSE_SCORE
        } else if second == "Y" {
            PAPER_SCORE + DRAW_SCORE
        } else {
            SCISSOR_SCORE + WIN_SCORE
        }
    } else {
        if second == "X" {
            PAPER_SCORE + LOSE_SCORE
        } else if second == "Y" {
            SCISSOR_SCORE + DRAW_SCORE
        } else {
            ROCK_SCORE + WIN_SCORE
        }
    }
}

fn main() {
    let path = "input/input.txt";
    let data = fs::read_to_string(path).expect("Error when readig the file");
    let mut first_score = 0;
    let mut second_score = 0;
    for line in data.lines() {
        let mut moves = line.split(" ");
        let first = moves.next().unwrap();
        let second = moves.next().unwrap();

        first_score += first_part_score(first, second);
        second_score += second_part_score(first, second);
    }
    println!("Result part 1: {}", first_score);
    println!("Result part 2: {}", second_score);
}
