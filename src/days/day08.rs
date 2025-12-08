use crate::{Solution, SolutionPair};
use partitions::PartitionVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point(i64, i64, i64);

impl Point {
    fn distance(p: &Point, q: &Point) -> f64 {
        let d = (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2);
        (d as f64).sqrt()
    }
}

fn prepare(input: &str) -> Vec<Point> {
    input
        .split_ascii_whitespace()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let mut xyz = line.split(',').map(|e| e.parse::<i64>().unwrap());
                let x = xyz.next().unwrap();
                let y = xyz.next().unwrap();
                let z = xyz.next().unwrap();
                Some(Point(x, y, z))
            }
        })
        .collect()
}

fn solve_part1(input: &str, cables: usize) -> usize {
    let jboxes = prepare(input);
    let mut partitions = PartitionVec::<Point>::new();
    jboxes.iter().for_each(|jbox| partitions.push(jbox.clone()));

    let mut distances = vec![];
    for i in 0..(jboxes.len() - 1) {
        let p = &jboxes[i];
        for j in (i + 1)..jboxes.len() {
            let q = &jboxes[j];
            let dist = Point::distance(p, q);
            distances.push((i, j, dist));
        }
    }

    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());
    for (i, j, _) in distances.iter().take(cables){
        partitions.union(*i, *j);
    }

    let mut sizes = partitions
        .all_sets()
        .map(|set| set.count())
        .collect::<Vec<_>>();
    sizes.sort();
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

fn solve_part2(input: &str) -> u64 {
    let jboxes = prepare(input);
    let mut partitions = PartitionVec::<Point>::new();
    jboxes.iter().for_each(|jbox| partitions.push(jbox.clone()));

    let mut distances = vec![];
    for i in 0..(jboxes.len() - 1) {
        let p = &jboxes[i];
        for j in (i + 1)..jboxes.len() {
            let q = &jboxes[j];
            let dist = Point::distance(p, q);
            distances.push((i, j, dist));
        }
    }

    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());
    for (i, j, _) in &distances {
        partitions.union(*i, *j);
        if partitions.amount_of_sets() == 1 {
            return u64::try_from(jboxes[*i].0 * jboxes[*j].0).unwrap();
        }
    }

    unreachable!()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input, 1000);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT, 10), 40);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 25272);
    }
}
