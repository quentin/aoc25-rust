mod days;
mod etc;

use days::*;
use etc::grid::{Grid, Point};
use etc::solution::Solution;
use std::env;

pub type SolutionPair = (Solution, Solution);

fn solve_day(day: u8) -> SolutionPair {
    let input = std::fs::read_to_string(format!("./input/day{:0>2}.txt", day)).unwrap();
    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        //9 => day09::solve(input),
        //10 => day10::solve(input),
        //11 => day11::solve(input),
        //12 => day12::solve(input),
        //13 => day13::solve(input),
        //14 => day14::solve(input),
        //15 => day15::solve(input),
        //16 => day16::solve(input),
        //17 => day17::solve(input),
        //18 => day18::solve(input),
        //19 => day19::solve(input),
        //20 => day20::solve(input),
        //21 => day21::solve(input),
        //22 => day22::solve(input),
        //23 => day23::solve(input),
        //24 => day24::solve(input),
        //25 => day25::solve(input),
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s)");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    for day in days {
        let (p1, p2) = solve_day(day);
        println!("\n=== Day {:02} ===", day);
        println!("   Part 1: {}", p1);
        println!("   Part 2: {}", p2);
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::solve_day;

    #[test]
    fn my_puzzles() {
        assert_eq!(
            solve_day(1),
            (Solution::from(1191u64), Solution::from(6858u64))
        );
        assert_eq!(
            solve_day(2),
            (
                Solution::from(40398804950u64),
                Solution::from(65794984339u64)
            )
        );
        assert_eq!(
            solve_day(3),
            (
                Solution::from(17435u64),
                Solution::from(172886048065379u64)
            )
        );
        assert_eq!(
            solve_day(4),
            (
                Solution::from(1419u64),
                Solution::from(8739usize)
            )
        );
        assert_eq!(
            solve_day(5),
            (
                Solution::from(652usize),
                Solution::from(341753674214273u64)
            )
        );
        assert_eq!(
            solve_day(6),
            (
                Solution::from(5784380717354u64),
                Solution::from(7996218225744u64)
            )
        );
        assert_eq!(
            solve_day(7),
            (
                Solution::from(1640usize),
                Solution::from(40999072541589u64)
            )
        );
        assert_eq!(
            solve_day(8),
            (
                Solution::from(63920usize),
                Solution::from(1026594680u64)
            )
        );
    }
}
