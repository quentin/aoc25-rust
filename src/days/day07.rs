use crate::{Solution, SolutionPair};

/// Return `(width, start, splitters)`.
fn prepare(input: &str) -> (usize, usize, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let start_line = lines.next().unwrap();
    let width = start_line.len();
    let start = start_line.find('S').unwrap();
    let mut splitters = vec![];
    while let Some(line) = lines.next() {
        let splitter_at = line
            .chars()
            .enumerate()
            .filter_map(|(pos, c)| if c == '^' { Some(pos) } else { None })
            .collect::<Vec<_>>();
        splitters.push(splitter_at);
    }

    (width, start, splitters)
}

fn solve_part1(input: &str) -> usize {
    let (width, start, splitters) = prepare(input);
    let mut beams = vec![false; width];

    let mut splits = 0;
    beams[start] = true;
    for level in splitters {
        let mut next_beams = beams.clone();
        for splitter in level {
            if beams[splitter] {
                next_beams[splitter] = false;
                next_beams[splitter - 1] = true;
                next_beams[splitter + 1] = true;
                splits += 1;
            }
        }
        beams = next_beams;
    }

    splits
}

fn solve_part2(input: &str) -> u64 {
    let (width, start, splitters) = prepare(input);
    let mut beams = vec![0u64; width];

    beams[start] = 1;
    for level in splitters {
        let mut next_beams = beams.clone();
        for splitter in level {
            if beams[splitter] > 0 {
                next_beams[splitter - 1] += beams[splitter];
                next_beams[splitter + 1] += beams[splitter];
                next_beams[splitter] = 0;
            }
        }
        beams = next_beams;
    }

    beams.iter().sum()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 40);
    }
}
