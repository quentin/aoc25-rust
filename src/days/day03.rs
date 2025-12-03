use crate::{Solution, SolutionPair};

/// Return an array of banks, each bank is an array of integers `1..=9`.
fn prepare(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn solve_part1(input: &str) -> u64 {
    // Find `a` the battery with greatest joltage of the bank that is not the last battery,
    // and `b` the battery after `a` with the greatest joltage in the bank.
    //
    // Be `j(a)` and `j(b)` the joltage of each battery, the largest voltage you can produce
    // is `10*j(a) + j(b)`.
    let banks = prepare(input);
    banks
        .into_iter()
        .map(|bank| -> u64 {
            let slice = &bank[0..bank.len() - 1];
            let j_a = *slice.iter().max().unwrap();
            let a = slice.iter().position(|jolt| *jolt == j_a).unwrap();
            let slice = &bank[a + 1..bank.len()];
            let j_b = *slice.iter().max().unwrap();
            (10 * j_a + j_b).into()
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let banks = prepare(input);
    banks
        .into_iter()
        .map(|bank| {
            let mut highest = 0;
            let mut start = 0;
            for d in (0usize..12).rev() {
                let slice = &bank[start..bank.len() - d];
                let j_i = *slice.iter().max().unwrap();
                start += slice.iter().position(|jolt| *jolt == j_i).unwrap() + 1;
                highest = 10 * highest + <u32 as Into<u64>>::into(j_i);
            }
            highest
        })
        .sum()
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
    987654321111111
    811111111111119
    234234234234278
    818181911112111";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 3121910778619);
    }
}
