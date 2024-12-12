/*

https://adventofcode.com/2024/day/7

part 1:

for each equation, we can exhaustively search all possible combinations of operations (2^N-1) and simply check the result.
For N=12 this is fine.

part 2:
adding a third operation but 3^(N-1) is still fine for N=12

takes 5 seconds but I imagine we could speed up a lot by reusing the computation when the prefix of operations is the same
(iterating through a tree where you pick 1 of 3 iterations at next edge). And a key property of all 3 operations is that the result only gets larger which means that
a lot of the tree doesn't need to be explored.

*/

fn main() {
    let input = parse(include_str!("../../input/07.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1153997401072);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 97902809384118);
}

#[derive(Debug)]
struct Equation {
    lhs: i64,
    rhs: Vec<i64>,
}
type Input = Vec<Equation>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let lhs = parts.next().unwrap().parse().unwrap();
            let rhs = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            Equation { lhs, rhs }
        })
        .collect()
}

fn num_solutions(equation: &Equation) -> i64 {
    let mut count = 0;
    // iterate all combinations of operations (2^N-1)
    let n = equation.rhs.len();
    for i in 0..(1 << n) {
        let mut sum = equation.rhs[0];
        for j in 0..n - 1 {
            if i & (1 << j) != 0 {
                sum += equation.rhs[j + 1];
            } else {
                sum *= equation.rhs[j + 1];
            }
        }
        if sum == equation.lhs {
            count += 1;
        }
    }
    count
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for equation in input {
        if num_solutions(&equation) >= 1 {
            sum += equation.lhs;
        }
    }
    sum
}

fn concat_op(a: i64, b: i64) -> Option<i64> {
    let a_str = a.to_string();
    let b_str = b.to_string();
    let c_str = a_str + &b_str;
    c_str.parse().ok()
}

fn num_solutions_part2(equation: &Equation) -> i64 {
    let n = equation.rhs.len();
    let mut count = 0;

    // 3^(N-1)
    let max = 3i64.pow(n as u32 - 1);

    for i in 0..max {
        let mut sum = equation.rhs[0];
        for j in 0..n - 1 {
            let op = (i / 3i64.pow(j as u32)) % 3;
            if op == 0 {
                sum += equation.rhs[j + 1];
            } else if op == 1 {
                sum *= equation.rhs[j + 1];
            } else {
                // concat
                match concat_op(sum, equation.rhs[j + 1]) {
                    Some(v) => sum = v,
                    // concat leads to overflow / invalid number
                    None => break,
                }
            }
        }
        if sum == equation.lhs {
            count += 1;
        }
    }
    count
}

fn part2(input: &Input) -> i64 {
    let mut sum = 0;
    for equation in input {
        if num_solutions_part2(&equation) >= 1 {
            sum += equation.lhs;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/07_example.txt"));
        assert_eq!(part1(&input), 3749);
        assert_eq!(part2(&input), 11387);
    }
}
