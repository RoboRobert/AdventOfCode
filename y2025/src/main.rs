mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let day = args.next().unwrap().parse().unwrap();
    let puzzle = args.next().unwrap().parse().unwrap();
    let input = fs::read_to_string(args.next().unwrap()).unwrap();
    let answer = match (day, puzzle) {
        (1, 1) => day01::puzzle1(&input).to_string(),
        (1, 2) => day01::puzzle2(&input).to_string(),
        (2, 1) => day02::puzzle1(&input).to_string(),
        (2, 2) => day02::puzzle2(&input).to_string(),
        (3, 1) => day03::puzzle1(&input).to_string(),
        (3, 2) => day03::puzzle2(&input).to_string(),
        (4, 1) => day04::puzzle1(&input).to_string(),
        (4, 2) => day04::puzzle2(&input).to_string(),
        (5, 1) => day05::puzzle1(&input).to_string(),
        (5, 2) => day05::puzzle2(&input).to_string(),
        (6, 1) => day06::puzzle1(&input).to_string(),
        (6, 2) => day06::puzzle2(&input).to_string(),

        _ => panic!("no puzzle {} for day {}", puzzle, day),
    };
    println!("{}", answer.trim_end());
}
