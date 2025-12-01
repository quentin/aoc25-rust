use crate::{Solution, SolutionPair};

const START_POSITION: i32 = 50;

fn prepare(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let v: i32 = line[1..].parse().unwrap();
            match &line[0..1] {
                "L" => -v,
                "R" => v,
                _ => unreachable!(),
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> u64 {
    let rotations = prepare(input);
    rotations
        .iter()
        .fold((START_POSITION, 0u64), |(pos, zeroes), rot| {
            let new_pos = (pos + rot) % 100;
            (new_pos, zeroes + if new_pos == 0 { 1 } else { 0 })
        })
        .1
}

fn solve_part2(input: &str) -> u64 {
    let rotations = prepare(input);
    rotations
        .iter()
        .fold((START_POSITION, 0u64), |(pos, zeroes), rot| {
            let new_pos = (pos + rot).rem_euclid(100);

            let full_turns: u64 = (rot.abs() / 100).try_into().unwrap();
            let resets: u64 = (new_pos == 0
                || (pos != 0 && ((*rot > 0 && new_pos < pos) || (*rot < 0 && new_pos > pos))))
                .into();
            (new_pos, zeroes + full_turns + resets)
        })
        .1
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

    #[test]
    fn preparation() {
        assert_eq!(
            prepare(EXAMPLE_INPUT),
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 6);
    }
}
