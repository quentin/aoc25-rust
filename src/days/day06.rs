use crate::{Solution, SolutionPair};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    operation: Op,
    operands: Vec<u64>,
}

impl Problem {
    fn eval(&self) -> u64 {
        match self.operation {
            Op::Add => self.operands.iter().copied().fold(0, |a, b| a + b),
            Op::Mul => self.operands.iter().copied().fold(1, |a, b| a * b),
        }
    }
}

fn read_operations(line: &str) -> Vec<Op> {
    line.split_ascii_whitespace()
        .map(|e| match e {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!("wrong operand {e}"),
        })
        .collect()
}

fn prepare(input: &str) -> Vec<Problem> {
    let mut lines = input.lines().collect::<Vec<_>>();

    let operations = read_operations(lines.pop().unwrap());

    let mut problems = operations
        .into_iter()
        .map(|operation| Problem {
            operation,
            operands: Default::default(),
        })
        .collect::<Vec<_>>();

    let numbers = |line: &str| {
        line.split_ascii_whitespace()
            .map(|e| e.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    };
    for line in lines {
        for (i, operand) in numbers(line).into_iter().enumerate() {
            problems[i].operands.push(operand);
        }
    }

    problems
}

fn solve_part1(input: &str) -> u64 {
    let problems = prepare(input);
    problems.into_iter().map(|problem| problem.eval()).sum()
}

fn prepare_transposed(input: &str) -> Vec<Problem> {
    let mut lines = input.lines().collect::<Vec<_>>();

    let mut operations = read_operations(lines.pop().unwrap());
    let mut lines = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut problems = vec![];
    let mut operands = vec![];
    // read columns from right to left
    for _ in 0..lines.first().unwrap().len() {
        let mut o = 0;
        for line in &mut lines {
            if let Some(d) = line.pop().unwrap().to_digit(10) {
                o = 10 * o + d;
            }
        }

        if o == 0 {
            // end of current problem when column is empty (zero)
            problems.push(Problem {
                operation: operations.pop().unwrap(),
                operands: operands.clone(),
            });
            operands.clear();
        } else {
            operands.push(o as u64);
        }
    }

    if !operands.is_empty() {
        // one last problem
        problems.push(Problem {
            operation: operations.pop().unwrap(),
            operands: operands.clone(),
        });
    }

    problems
}

fn solve_part2(input: &str) -> u64 {
    let problems = prepare_transposed(input);
    problems.into_iter().map(|problem| problem.eval()).sum()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 3263827);
    }
}
