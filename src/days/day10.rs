use crate::{Solution, SolutionPair};

#[derive(Debug, PartialEq, Eq)]
struct InitProcedure {
    /// required indicator lights stored as a binary representation where i-th bit is ON if the
    /// i-th indicator light is required to be ON.
    required_indicator_lights: u64,

    /// each button is stored as a binary representation where i-th bit is ON if this button
    /// toggles the i-th indicator light.
    toggle_buttons: Vec<u64>,

    specified_joltage_levels: Vec<u64>,
}

fn prepare(input: &str) -> Vec<InitProcedure> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| {
            let rbrk = line.find(']').unwrap();
            let mut required_indicator_lights = 0u64;
            for c in (&line[1..rbrk]).chars().rev() {
                match c {
                    '.' => required_indicator_lights = required_indicator_lights << 1,
                    '#' => required_indicator_lights = (required_indicator_lights << 1) + 1,
                    ']' => break,
                    _ => unreachable!(),
                }
            }

            let re = regex::Regex::new(r"\(([0-9,]+)\)").unwrap();
            let toggle_buttons = re
                .captures_iter(line)
                .map(|cap| {
                    cap.get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|digits| digits.parse::<u64>().unwrap())
                        .fold(0u64, |acc, num| acc + (1u64 << num))
                })
                .collect();

            let re = regex::Regex::new(r"\{([0-9,]+)\}").unwrap();
            let specified_joltage_levels = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|digits| digits.parse::<u64>().unwrap())
                .collect();

            InitProcedure {
                required_indicator_lights,
                toggle_buttons,
                specified_joltage_levels,
            }
        })
        .collect()
}

/// Return the smallest number of buttons to push in order to perform the initialization procedure.
///
/// It's a breadth-first exploration of the reachable indicator lights states.
fn init_steps(init: &InitProcedure) -> u64 {
    let mut worklist = std::collections::VecDeque::from([0u64]);
    // assume there are no more than 11 indicator lights.
    let mut shortest_path = vec![0; 1 << 12];
    let target = init.required_indicator_lights;

    while let Some(s) = worklist.pop_front() {
        let sp = shortest_path[s as usize];
        if s == target {
            return sp;
        }
        for toggle in &init.toggle_buttons {
            let n = s ^ toggle;
            if shortest_path[n as usize] == 0 {
                shortest_path[n as usize] = sp + 1;
                worklist.push_back(n);
            }
        }
    }

    unreachable!()
}

fn solve_part1(input: &str) -> u64 {
    let inits = prepare(input);
    inits.into_iter().map(|init| init_steps(&init)).sum()
}

type Joltage = u64;
type Button = usize;
type Presses = u64;

/// Equation of a counter
#[derive(Debug)]
struct Equation {
    /// Counter's target joltage
    joltage: Joltage,

    /// buttons that increment this counter
    buttons: Vec<Button>,
}

/// Return the allowed range for the variable.
fn variable_range(
    equations: &Vec<Equation>,
    environ: &[Option<Presses>],
    var: Button,
) -> Option<(Button, std::ops::RangeInclusive<Joltage>)> {
    assert!(!environ[var].is_some());
    let mut min: Option<Presses> = None;
    let mut max: Option<Presses> = None;

    equations.iter().for_each(|eq| {
        if eq.buttons.contains(&var) {
            if let Some(local_max) = eq
                .joltage
                .checked_sub(eq.buttons.iter().map(|b| environ[*b].unwrap_or(0)).sum())
            {
                max = Some((*max.get_or_insert(local_max)).min(local_max));
                if eq
                    .buttons
                    .iter()
                    .all(|b| *b == var || environ[*b].is_some())
                {
                    // all buttons in the equation are bound except this one, so we can deduce a
                    // min presses for this button
                    min = Some((*min.get_or_insert(local_max)).max(local_max));
                }
            }
        }
    });
    match (min, max) {
        (None, None) => None,
        (None, Some(hi)) => Some((var, (0..=hi))),
        (Some(lo), Some(hi)) if lo <= hi => Some((var, (lo..=hi))),
        _ => None,
    }
}

/// Recursively find the least number of button presses that solves the equations.
fn search_least_presses(equations: &Vec<Equation>, environ: &mut [Option<u64>]) -> Option<u64> {
    // select the free button (variable) with the smallest allowed range
    let mut best_var_and_range = None;
    for var in 0..environ.len() {
        if environ[var].is_some() {
            continue;
        }
        if let Some((var, range)) = variable_range(equations, environ, var) {
            best_var_and_range = match (best_var_and_range, range) {
                (None, range) => Some((var, range)),
                (Some((_, best_range)), range)
                    if (best_range.end() - best_range.start()) > (range.end() - range.start()) =>
                {
                    Some((var, range))
                }
                (b,_) => b,
            };
        } else {
            return None;
        }
    }

    if let Some((var, range)) = best_var_and_range {
        let mut local_least = None;
        for presses in range.rev() {
            environ[var] = Some(presses);
            if let Some(other_local_least) = search_least_presses(equations, environ) {
                local_least = Some(local_least.unwrap_or(u64::max_value()).min(other_local_least));
            }
        }
        environ[var] = None;
        return local_least;
    } else {
        // no free variable, we are done
        let score = environ.iter().map(|maybe| maybe.unwrap()).sum();
        return Some(score);
    }
}

fn solve_joltages(init: &InitProcedure) -> u64 {
    let button_count = init.toggle_buttons.len();
    let mut equations = init
        .specified_joltage_levels
        .iter()
        .map(|joltage| Equation {
            joltage: *joltage,
            buttons: vec![],
        })
        .collect::<Vec<_>>();
    init.toggle_buttons
        .iter()
        .enumerate()
        .for_each(|(button, pattern)| {
            for i in 0..init.specified_joltage_levels.len() {
                if (pattern & (1 << i)) != 0 {
                    equations[i].buttons.push(button);
                }
            }
        });
    let best = search_least_presses(&equations, &mut vec![None; button_count]).unwrap();
    eprintln!("{equations:?}\n=> {best}");
    best
}

fn solve_part2(input: &str) -> u64 {
    let inits = prepare(input);
    inits.iter().map(|init| solve_joltages(init)).sum()
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
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn preparation() {
        let inits = prepare(EXAMPLE_INPUT);
        assert_eq!(
            inits[0],
            InitProcedure {
                required_indicator_lights: 6,
                toggle_buttons: vec![
                    (1 << 3),
                    (1 << 1) + (1 << 3),
                    (1 << 2),
                    (1 << 2) + (1 << 3),
                    (1 << 0) + (1 << 2),
                    (1 << 0) + (1 << 1)
                ],
                specified_joltage_levels: vec![3, 5, 4, 7]
            }
        );
        assert_eq!(inits[1].required_indicator_lights, 1u64 << 3);
        assert_eq!(
            inits[2].required_indicator_lights,
            (1 << 1) + (1 << 2) + (1 << 3) + (1 << 5)
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 33);
    }
}
