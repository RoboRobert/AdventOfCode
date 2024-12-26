mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        (7, 1) => day07::puzzle1(&input).to_string(),
        (7, 2) => day07::puzzle2(&input).to_string(),
        (8, 1) => day08::puzzle1(&input).to_string(),
        (8, 2) => day08::puzzle2(&input).to_string(),
        (9, 1) => day09::puzzle1(&input).to_string(),
        (9, 2) => day09::puzzle2(&input).to_string(),
        (10, 1) => day10::puzzle1(&input).to_string(),
        (10, 2) => day10::puzzle2(&input).to_string(),
        (11, 1) => day11::puzzle1(&input).to_string(),
        (11, 2) => day11::puzzle2(&input).to_string(),
        (12, 1) => day12::puzzle1(&input).to_string(),
        (12, 2) => day12::puzzle2(&input).to_string(),
        (13, 1) => day13::puzzle1(&input).to_string(),
        (13, 2) => day13::puzzle2(&input).to_string(),
        (14, 1) => day14::puzzle1(&input).to_string(),
        (14, 2) => day14::puzzle2(&input).to_string(),
        (15, 1) => day15::puzzle1(&input).to_string(),
        (15, 2) => day15::puzzle2(&input).to_string(),
        (16, 1) => day16::puzzle1(&input).to_string(),
        (16, 2) => day16::puzzle2(&input).to_string(),
        (17, 1) => day17::puzzle1(&input).to_string(),
        (17, 2) => day17::puzzle2(&input).to_string(),
        (18, 1) => day18::puzzle1(&input).to_string(),
        (18, 2) => day18::puzzle2(&input).to_string(),
        (19, 1) => day19::puzzle1(&input).to_string(),
        (19, 2) => day19::puzzle2(&input).to_string(),
        (20, 1) => day20::puzzle1(&input).to_string(),
        (20, 2) => day20::puzzle2(&input).to_string(),
        (21, 1) => day21::puzzle1(&input).to_string(),
        (21, 2) => day21::puzzle2(&input).to_string(),
        (22, 1) => day22::puzzle1(&input).to_string(),
        (22, 2) => day22::puzzle2(&input).to_string(),
        (23, 1) => day23::puzzle1(&input).to_string(),
        (23, 2) => day23::puzzle2(&input).to_string(),
        (24, 1) => day24::puzzle1(&input).to_string(),
        (24, 2) => day24::puzzle2(&input).to_string(),
        (25, 1) => day25::puzzle1(&input).to_string(),
        (25, 2) => day25::puzzle2(&input).to_string(),

        _ => panic!("no puzzle {} for day {}", puzzle, day),
    };
    println!("{}", answer.trim_end());
}