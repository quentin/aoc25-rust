use crate::{Solution, SolutionPair};
use std::ops::RangeInclusive;

/// Fresh ranges are sorted by increasing start and decreasing end.
/// Ingredients are sorted in increasing order.
fn prepare(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut fresh_ranges = vec![];
    let mut ingredients = vec![];

    let lines = input.lines().map(|line| line.trim());
    let mut lines = lines.skip_while(|line| line.is_empty());
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (low, high) = line.split_once('-').unwrap();
        fresh_ranges.push(RangeInclusive::new(
            low.parse().unwrap(),
            high.parse().unwrap(),
        ));
    }

    while let Some(line) = lines.next() {
        ingredients.push(line.parse().unwrap());
    }

    fresh_ranges.sort_by(|ra, rb| {
        if ra.start() < rb.start() {
            std::cmp::Ordering::Less
        } else if ra.start() > rb.start() {
            std::cmp::Ordering::Greater
        } else if ra.end() > rb.end() {
            std::cmp::Ordering::Less
        } else if ra.end() < rb.end() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    ingredients.sort();

    (fresh_ranges, ingredients)
}

fn solve_part1(input: &str) -> usize {
    let (fresh_ranges, ingredients) = prepare(input);
    let mut fresh_range = fresh_ranges.iter();
    let mut ingredient = ingredients.iter();
    let mut fresh_count = 0;

    let mut current_range = fresh_range.next();
    while let Some(i) = ingredient.next() {
        while let Some(r) = current_range {
            if r.contains(i) {
                fresh_count += 1;
                break;
            } else if i < r.start() {
                // ingredient is spoiled
                break;
            } else if i > r.end() {
                // must look at next range
                current_range = fresh_range.next();
            }
        }
    }

    fresh_count
}

fn merge_ranges(mut input: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut i = 0;
    while i < input.len() && i + 1 < input.len() {
        let r = &input[i];
        let r2 = &input[i + 1];
        if r.end() < r2.start() {
            // non-overlapping
            i = i + 1;
        } else {
            // overlapping
            input[i] = RangeInclusive::new(*r.start(), std::cmp::max(*r.end(), *r2.end()));
            input.remove(i + 1);
        }
    }
    input
}

fn solve_part2(input: &str) -> u64 {
    let (fresh_ranges, _ingredients) = prepare(input);
    let fresh_ranges = merge_ranges(fresh_ranges);
    fresh_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
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
    3-5
    10-14
    16-20
    12-18

    1
    17
    8
    11
    5
    32";

    #[test]
    fn preparation() {
        let (fresh_ranges, ingredients) = prepare(EXAMPLE_INPUT);
        assert_eq!(
            fresh_ranges,
            vec![
                RangeInclusive::new(3, 5),
                RangeInclusive::new(10, 14),
                RangeInclusive::new(12, 18),
                RangeInclusive::new(16, 20)
            ]
        );
        assert_eq!(ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 14u64);
    }
}
