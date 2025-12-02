use crate::{Solution, SolutionPair};

use std::ops::RangeInclusive;

fn prepare(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|txt| {
            let pair = txt.split_once('-').unwrap();
            RangeInclusive::new(pair.0.parse().unwrap(), pair.1.parse().unwrap())
        })
        .collect()
}

fn pattern_repeats_twice(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    let len = n.ilog10() + 1;
    let half_len = len / 2;
    return (len % 2 == 0) && n.rem_euclid(1 + 10u64.pow(half_len)) == 0;
}

fn solve_part1(input: &str) -> u64 {
    prepare(input)
        .iter()
        .map(|range| {
            range
                .clone()
                .filter(|n| pattern_repeats_twice(*n))
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn pattern_repeats_k(k: u64, n: u64) -> bool {
    // `n` must be divisible by decimal pattern `Z1...Z1` where `Z` is `0` repeated `m-1` times
    // and `Z1` is repeated `k` times
    if k == 0 || n == 0 {
        return false;
    }
    let len: u64 = (n.ilog10() + 1).into();
    if len.rem_euclid(k) != 0 {
        return false;
    }
    let m: u32 = (len / k).try_into().unwrap();
    let mut p = 0;
    for _ in 0..k {
        p = 1 + p * 10u64.pow(m);
    }
    return n.rem_euclid(p) == 0;
}

fn pattern_repeats_at_least_twice(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    let len: u64 = (n.ilog10() + 1).into();
    for k in 2..=len {
        if pattern_repeats_k(k, n) {
            return true;
        }
    }
    return false;
}

fn solve_part2(input: &str) -> u64 {
    prepare(input)
        .iter()
        .map(|range| {
            range
                .clone()
                .filter(|n| pattern_repeats_at_least_twice(*n))
                .sum::<u64>()
        })
        .sum::<u64>()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn preparation() {
        assert_eq!(
            prepare("11-22,95-115,998-1012"),
            vec![11u64..=22, 95u64..=115, 998u64..=1012]
        );
    }

    #[test]
    fn repeat_twice() {
        assert_eq!(pattern_repeats_twice(0), false);
        assert_eq!(pattern_repeats_twice(1), false);
        assert_eq!(pattern_repeats_twice(10), false);
        assert_eq!(pattern_repeats_twice(11), true);
        assert_eq!(pattern_repeats_twice(99), true);
        assert_eq!(pattern_repeats_twice(101), false);
        assert_eq!(pattern_repeats_twice(110), false);
        assert_eq!(pattern_repeats_twice(1010), true);
        assert_eq!(pattern_repeats_twice(1210), false);
        assert_eq!(pattern_repeats_twice(12991299), true);
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn repeat_k() {
        assert_eq!(pattern_repeats_k(2, 0), false);
        assert_eq!(pattern_repeats_k(2, 1), false);
        assert_eq!(pattern_repeats_k(2, 10), false);
        assert_eq!(pattern_repeats_k(2, 11), true);
        assert_eq!(pattern_repeats_k(2, 99), true);
        assert_eq!(pattern_repeats_k(2, 101), false);
        assert_eq!(pattern_repeats_k(2, 110), false);
        assert_eq!(pattern_repeats_k(2, 1010), true);
        assert_eq!(pattern_repeats_k(2, 1210), false);
        assert_eq!(pattern_repeats_k(2, 12991299), true);

        assert!(pattern_repeats_k(1, 99));
        assert!(pattern_repeats_k(2, 99));
        assert!(!pattern_repeats_k(3, 99));

        assert!(pattern_repeats_k(1, 111));
        assert!(!pattern_repeats_k(2, 111));
        assert!(pattern_repeats_k(3, 111));

        assert!(pattern_repeats_k(1, 222222));
        assert!(pattern_repeats_k(2, 222222));
        assert!(pattern_repeats_k(3, 222222));
        assert!(pattern_repeats_k(6, 222222));

        assert!(!pattern_repeats_k(2, 565656));
        assert!(pattern_repeats_k(3, 565656));
        assert!(!pattern_repeats_k(4, 565656));
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 4174379265u64);
    }
}
