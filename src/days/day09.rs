use crate::{Solution, SolutionPair};

#[derive(Copy, Clone, Debug)]
struct Tile(i64, i64);

fn prepare(input: &str) -> Vec<Tile> {
    input
        .split_ascii_whitespace()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                line.split_once(',')
            }
        })
        .map(|(a, b)| Tile(a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

/// Take two opposite corners of a rectangle and return the top-left and bottom-left corners.
fn normalize_corners(a: &Tile, b: &Tile) -> (Tile, Tile) {
    let top_left = Tile(a.0.min(b.0), a.1.min(b.1));
    let bottom_right = Tile(a.0.max(b.0), a.1.max(b.1));
    (top_left, bottom_right)
}

/// Generate all the rectangles as coordinates of the top-left and bottom-right corners and its
/// area `(corner1, corner2, area)`. The rectangles are sorted by increasing area.
fn all_rectangles(red_tiles: &[Tile]) -> Vec<(Tile, Tile, u64)> {
    let mut rectangles = vec![];
    for i in 0..(red_tiles.len() - 1) {
        for j in i..red_tiles.len() {
            let a = red_tiles[i];
            let b = red_tiles[j];
            let (top_left, bottom_right) = normalize_corners(&a, &b);
            let area = u64::try_from(
                ((bottom_right.0 - top_left.0).abs() + 1)
                    * ((bottom_right.1 - top_left.1).abs() + 1),
            )
            .unwrap();
            rectangles.push((top_left, bottom_right, area));
        }
    }
    rectangles.sort_by_key(|(_, _, area)| *area);
    rectangles
}

fn solve_part1(input: &str) -> u64 {
    // return the area of the largest rectangle
    let red_tiles = prepare(input);
    all_rectangles(&red_tiles).pop().unwrap().2
}

/// Generate all the segments of the outer shape.
///
/// Segment's endpoint are normalized as `(top-left, bottom-right)`.
fn all_segments(red_tiles: &[Tile]) -> Vec<(Tile, Tile)> {
    let mut segments = red_tiles
        .windows(2)
        .map(|arr| {
            let a = arr[0];
            let b = arr[1];
            normalize_corners(&a, &b)
        })
        .collect::<Vec<_>>();
    segments.push(normalize_corners(
        red_tiles.first().unwrap(),
        red_tiles.last().unwrap(),
    ));
    segments
}

/// Test strict intersection of segment with a rectangle.
///
/// Segments are expected to be either vertical or horizontal.
fn segment_intersects_rectangle(segment: &(Tile, Tile), c1: &Tile, c2: &Tile) -> bool {
    let s1 = &segment.0;
    let s2 = &segment.1;
    // horizontal segment crossing the rectangle ?
    if s1.1 == s2.1 && s1.1 > c1.1 && s2.1 < c2.1 && s1.0 < c2.0 && s2.0 > c1.0 {
        return true;
    }
    // vertical segment crossing the rectangle ?
    if s1.0 == s2.0 && s1.0 > c1.0 && s2.0 < c2.0 && s1.1 < c2.1 && s2.1 > c1.1 {
        return true;
    }
    return false;
}

fn solve_part2(input: &str) -> u64 {
    let red_tiles = prepare(input);
    let rectangles = all_rectangles(&red_tiles);
    let segments = all_segments(&red_tiles);
    // Return the area of the largest rectangle that has no red-red segment crossing its borders.
    //
    // Test from largest rectangle to smallest rectangle.
    rectangles
        .into_iter()
        .rev()
        .find_map(|(c1, c2, area)| {
            if segments
                .iter()
                .any(|segment| segment_intersects_rectangle(segment, &c1, &c2))
            {
                None
            } else {
                Some(area)
            }
        })
        .unwrap()
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
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 50);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 24);
    }
}
