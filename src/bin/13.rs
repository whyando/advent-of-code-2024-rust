/*

https://adventofcode.com/2024/day/13

part 1:
The hint is that less than 100 presses are needed to it's trival to brute force 100*100 combinations for each machine

part 2:
The brute force way doesn't work but we can reduce the problem to a system of two linear equations
Then if det!=0 there is a unique solution, (which we check is integer), or if det=0 there is no solution or there are infinite solutions
This case didn't happen in the input.

If this were to occur, we then we'd quickly check if one equation was a (rational) multiple of the other to know if we have infinite solutions

We then drop one of the dimensions but this case is kinda annoying because we have to pick out integer solutions from the infinite solutions


*/

fn main() {
    let input = parse(include_str!("../../input/13.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 25751);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 108528956728655);
}

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

type Input = Vec<Machine>;

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let mut machines = Vec::new();

    loop {
        let button_a_line = lines.next().unwrap();
        let button_b_line = lines.next().unwrap();
        let prize_line = lines.next().unwrap();

        // parse button_a using regex
        let button_a = {
            let re = regex::Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
            let caps = re.captures(button_a_line).unwrap();
            (caps[1].parse().unwrap(), caps[2].parse().unwrap())
        };
        let button_b = {
            let re = regex::Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
            let caps = re.captures(button_b_line).unwrap();
            (caps[1].parse().unwrap(), caps[2].parse().unwrap())
        };
        let prize = {
            let re = regex::Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
            let caps = re.captures(prize_line).unwrap();
            (caps[1].parse().unwrap(), caps[2].parse().unwrap())
        };
        machines.push(Machine {
            button_a,
            button_b,
            prize,
        });
        if lines.next().is_none() {
            break;
        }
    }
    machines
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for machine in input {
        let mut min_token_cost = None;
        for a_presses in 0..=100 {
            for b_presses in 0..=100 {
                let x = machine.button_a.0 * a_presses + machine.button_b.0 * b_presses;
                let y = machine.button_a.1 * a_presses + machine.button_b.1 * b_presses;
                if x == machine.prize.0 && y == machine.prize.1 {
                    let token_cost = a_presses * 3 + b_presses;
                    match min_token_cost {
                        None => min_token_cost = Some(token_cost),
                        Some(min) => min_token_cost = Some(min.min(token_cost)),
                    }
                }
            }
        }
        if let Some(min_token_cost) = min_token_cost {
            sum += min_token_cost;
        }
    }
    sum
}

fn part2(input: &Input) -> i64 {
    let mut sum = 0;
    for machine in input {
        let prize_x = machine.prize.0 + 10000000000000;
        let prize_y = machine.prize.1 + 10000000000000;
        println!(
            "{}a + {}b = {}",
            machine.button_a.0, machine.button_b.0, prize_x
        );
        println!(
            "{}a + {}b = {}",
            machine.button_a.1, machine.button_b.1, prize_y
        );

        let det = machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0;
        if det == 0 {
            panic!("det = 0");
        }
        let a_det = prize_x * machine.button_b.1 - prize_y * machine.button_b.0;
        let b_det = machine.button_a.0 * prize_y - machine.button_a.1 * prize_x;
        if a_det % det != 0 || b_det % det != 0 {
            println!("No integer solution");
        } else {
            let a = a_det / det;
            let b = b_det / det;
            println!("a = {}, b = {}", a, b);
            sum += a * 3 + b;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/13_example.txt"));
        assert_eq!(part1(&input), 480);
        assert_eq!(part2(&input), 875318608908);
    }
}
