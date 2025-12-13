use crate::{Solution, SolutionPair};

#[derive(Debug, PartialEq)]
struct Region {
    width: u64,
    height: u64,
    shapes: Vec<u64>,
}

type Area = u64;

fn prepare(input: &str) -> (Vec<Area>, Vec<Region>) {
    // parse area of each shape
    let mut areas = vec![];
    let re = regex::Regex::new(r"[#\.]{3}").unwrap();
    let mut it = re.captures_iter(input);
    while let Some(l0) = it.next() {
        let c0 = l0
            .get(0)
            .unwrap()
            .as_str()
            .chars()
            .filter(|c| *c == '#')
            .count();
        let c1 = it
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .chars()
            .filter(|c| *c == '#')
            .count();
        let c2 = it
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .chars()
            .filter(|c| *c == '#')
            .count();
        areas.push((c0 + c1 + c2) as u64);
    }

    // parse regions
    let re = regex::Regex::new(r"(\d+)x(\d+): ([\d ]+)").unwrap();
    let regions = re
        .captures_iter(input)
        .map(|caps| {
            let width = caps.get(1).unwrap().as_str().parse().unwrap();
            let height = caps.get(2).unwrap().as_str().parse().unwrap();
            let shapes = caps
                .get(3)
                .unwrap()
                .as_str()
                .trim()
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect();

            Region {
                width,
                height,
                shapes,
            }
        })
        .collect();
    (areas, regions)
}

fn solve_part1(input: &str) -> usize {
    let (areas, regions) = prepare(input);
    regions
        .into_iter()
        .filter(|region| {
            let area = region.width * region.height;

            if region.width * region.height >= 9u64 * region.shapes.iter().sum::<u64>() {
                return true
            }

            let shape_area = region
                    .shapes
                    .iter()
                    .enumerate()
                    .map(|(shape, count)| count * areas[shape])
                    .sum();
            if area
                < shape_area
            {
                false
            } else {
                todo!("complicated")
            }
        })
        .count()
}

/// No part 2
fn solve_part2(_input: &str) -> u64 {
    0
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
    0:
    ###
    ##.
    ##.
    
    1:
    ###
    ##.
    .##
    
    2:
    .##
    ###
    ##.
    
    3:
    ##.
    ###
    ##.
    
    4:
    ###
    #..
    ###
    
    5:
    ###
    .#.
    ###
    
    4x4: 0 0 0 0 2 0
    12x5: 1 0 1 0 2 2
    12x5: 1 0 1 0 3 2";

    #[test]
    fn preparation() {
        let (_areas, regions) = prepare(EXAMPLE_INPUT);
        assert_eq!(regions.len(), 3);
        assert_eq!(
            regions[0],
            Region {
                width: 4,
                height: 4,
                shapes: vec![0, 0, 0, 0, 2, 0]
            }
        );
        assert_eq!(
            regions[1],
            Region {
                width: 12,
                height: 5,
                shapes: vec![1, 0, 1, 0, 2, 2]
            }
        );
    }
}
