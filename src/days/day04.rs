use crate::{Grid, Point};
use crate::{Solution, SolutionPair};

#[derive(Copy, Clone, Debug)]
enum Cell {
    Free,
    Roll,
}

type Map = Grid<Cell>;

fn prepare(input: &str) -> Map {
    Grid::new(input).new_from(|c| match c {
        '.' => Cell::Free,
        '@' => Cell::Roll,
        _ => unreachable!(),
    })
}

fn is_accessible_roll(map: &Map, pos: &Point) -> bool {
    if matches!(map.unchecked_get(pos), Cell::Free) {
        return false;
    }

    let mut adjacent_rolls = 0;
    map.for_each_tchebychev_neighbour(pos, |_, neighbour| {
        if matches!(neighbour, Cell::Roll) {
            adjacent_rolls += 1;
        }
    });

    adjacent_rolls < 4
}

fn solve_part1(input: &str) -> u64 {
    let map = prepare(input);
    let mut accessible_rolls = 0u64;
    map.for_each_with_position(|origin, _| {
        if is_accessible_roll(&map, &origin) {
            accessible_rolls += 1;
        }
    });
    accessible_rolls
}

fn solve_part2(input: &str) -> usize {
    let mut map = prepare(input);
    let rolls_before = map.iter().filter(|cell| matches!(cell, Cell::Roll)).count();
    loop {
        let mut removable: Vec<Point> = Default::default();
        map.for_each_with_position(|pos, _| {
            if is_accessible_roll(&map, &pos) {
                removable.push(pos);
            }
        });
        if removable.is_empty() {
            break;
        }
        for pos in &removable {
            *map.get_mut(pos).unwrap() = Cell::Free;
        }
    }

    let rolls_after = map.iter().filter(|cell| matches!(cell, Cell::Roll)).count();
    rolls_before - rolls_after
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
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 43);
    }
}
