/*

https://adventofcode.com/2024/day/17

part1:
fun implementation problem, like the intcode computer from previous years

part2:
probably the hardest problem yet. Brute force is too slow so you actually need to read the binary instructions
to figure out what the input program does and then reverse engineer it to find the correct input

But even that is a bit fiddly because it's not clear that the output is actually reversable, and whether it's unique.

The key realisations are that r_a is processed 3 bits at a time (beginning from the least significant bits),
and produces 1 output per loop of this processing.

Also that this value depends only on the higher bits, and not the lower bits. So we can figure out options for the least bits
of r_a first, and then work our way up.

*/

use regex::Regex;

fn main() {
    let input = parse(include_str!("../../input/17.txt"));

    let part1 = part1(&input).output;
    println!("Part 1: {}", part1);
    assert_eq!(part1, "1,4,6,1,6,4,3,0,3");

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 265061364597659);
}

#[derive(Debug)]
struct Input {
    ra: i64,
    rb: i64,
    rc: i64,
    program: Vec<i64>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let re = Regex::new(r"^Register (\w): (\d+)$").unwrap();
    let ra = re.captures(lines.next().unwrap()).unwrap()[2]
        .parse()
        .unwrap();
    let rb = re.captures(lines.next().unwrap()).unwrap()[2]
        .parse()
        .unwrap();
    let rc = re.captures(lines.next().unwrap()).unwrap()[2]
        .parse()
        .unwrap();

    let regex = Regex::new(r"Program: (.+)").unwrap();
    let _ = lines.next();
    let program = regex.captures(lines.next().unwrap()).unwrap()[1]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    Input {
        ra,
        rb,
        rc,
        program,
    }
}

fn combo_operand(operand: i64, ra: i64, rb: i64, rc: i64) -> i64 {
    match operand {
        0..=3 => operand,
        4 => ra,
        5 => rb,
        6 => rc,
        7 => panic!("reserved combo operand"),
        _ => panic!("Unknown operand: {}", operand),
    }
}

#[derive(Debug)]
struct Result {
    output: String,
    out: Vec<i64>,
    #[allow(dead_code)]
    ra: i64,
    #[allow(dead_code)]
    rb: i64,
    #[allow(dead_code)]
    rc: i64,
}

fn part1(input: &Input) -> Result {
    let mut ra = input.ra;
    let mut rb = input.rb;
    let mut rc = input.rc;
    let mut ip = 0;
    let mut output = vec![];
    for _i in 0.. {
        if ip + 1 >= input.program.len() as i64 {
            break;
        }
        let opcode = input.program[ip as usize];
        let operand = input.program[ip as usize + 1];
        // println!("Step: {}", i);
        // println!("ip: {}, opcode: {}, operand: {}", ip, opcode, operand);
        // println!("registers: ra: {}, rb: {}, rc: {}", ra, rb, rc);
        match opcode {
            0 => {
                // adv
                let combo_operand = combo_operand(operand, ra, rb, rc);
                let denom = 2i64.pow(combo_operand as u32);
                let division = (ra as f64) / (denom as f64);
                // println!("adv: {} / 2^{} = {}", ra, combo_operand, division);
                ra = division as i64;
                ip += 2;
            }
            1 => {
                // bxl
                let result = rb ^ operand;
                // println!("bxl: {} ^ {} = {}", rb, operand, result);
                rb = result;
                ip += 2;
            }
            2 => {
                // bst
                let combo_operand = combo_operand(operand, ra, rb, rc);
                let result = combo_operand & 0b111;
                // println!("bst: {} & 0b111 = {}", combo_operand, result);
                rb = result;
                ip += 2;
            }
            3 => {
                // jnz
                if ra != 0 {
                    // println!("jnz: {} != 0, ip = {}", ra, operand);
                    ip = operand;
                } else {
                    // println!("jnz: {} == 0", ra);
                    ip += 2;
                }
            }
            4 => {
                // bxc
                let result = rb ^ rc;
                // println!("bxc: {} ^ {} = {}", rb, rc, result);
                rb = result;
                ip += 2;
            }
            5 => {
                // out
                let combo_operand = combo_operand(operand, ra, rb, rc);
                let result = combo_operand & 0b111;
                // println!("out: {} & 0b111 = {}", combo_operand, result);
                output.push(result);
                ip += 2;
            }
            6 => {
                // bdv
                let combo_operand = combo_operand(operand, ra, rb, rc);
                let denom = 2i64.pow(combo_operand as u32);
                let division = (ra as f64) / (denom as f64);
                // println!("bdv: {} / 2^{} = {}", ra, combo_operand, division);
                rb = division as i64;
                ip += 2;
            }
            7 => {
                // cdv
                let combo_operand = combo_operand(operand, ra, rb, rc);
                let denom = 2i64.pow(combo_operand as u32);
                let division = (ra as f64) / (denom as f64);
                // println!("cdv: {} / 2^{} = {}", ra, combo_operand, division);
                rc = division as i64;
                ip += 2;
            }
            _ => panic!("Unknown opcode: {}", opcode),
        }
        // println!();
    }

    let output_str = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    Result {
        output: output_str,
        out: output,
        ra,
        rb,
        rc,
    }
}

fn f(input: &Input, i: usize, ra: i64, solutions: &mut Vec<i64>) {
    println!("f({}, {})", i, ra);

    let target = input.program[i..].to_vec();
    let mut k_candidates = vec![];
    for k in 0..=7 {
        let ra1 = (ra << 3) | k;
        let input = Input {
            ra: ra1,
            rb: 0,
            rc: 0,
            program: input.program.clone(),
        };
        let result = part1(&input);
        // println!("{} -> {:?}", k, result.out);
        if result.out == target {
            println!("Found k candidate: {}", k);
            k_candidates.push(k);
        }
    }
    // let k = k_candidates.last().unwrap();
    // println!("Picked k: {}", k);
    // let ra1 = (ra << 3) | k;
    // f(input, i - 1, ra1);
    if i == 0 {
        for k in k_candidates {
            let ra1 = (ra << 3) | k;
            println!("Final solution: {}", ra1);
            solutions.push(ra1);
        }
    } else {
        for k in k_candidates {
            let ra1 = (ra << 3) | k;
            f(input, i - 1, ra1, solutions);
        }
    }
}

fn part2(input: &Input) -> i64 {
    // lets solve one instruction at a time, starting from the final one
    println!("Program: {:?}", input.program);
    let mut solutions = vec![];
    f(input, input.program.len() - 1, 0, &mut solutions);
    // take min
    *solutions.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let input = Input {
            ra: 2024,
            rb: 0,
            rc: 0,
            program: vec![0, 3, 5, 4, 3, 0],
        };
        let result = part2(&input);
        assert_eq!(result, 117440);
    }

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/17_example.txt"));
        assert_eq!(part1(&input).output, "4,6,3,5,6,3,5,2,1,0");
        // assert_eq!(part2(&input), 0);
    }

    #[test]
    fn example_1() {
        let input = Input {
            ra: 0,
            rb: 0,
            rc: 9,
            program: vec![
                // bst rc
                2, 6,
            ],
        };
        assert_eq!(part1(&input).rb, 1);
    }

    #[test]
    fn example_2() {
        let input = Input {
            ra: 10,
            rb: 0,
            rc: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };
        assert_eq!(part1(&input).output, "0,1,2");
    }

    #[test]
    fn example_3() {
        let input = Input {
            ra: 2024,
            rb: 0,
            rc: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(part1(&input).output, "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn example_4() {
        let input = Input {
            ra: 0,
            rb: 29,
            rc: 0,
            program: vec![1, 7],
        };
        assert_eq!(part1(&input).rb, 26);
    }

    #[test]
    fn example_5() {
        let input = Input {
            ra: 0,
            rb: 2024,
            rc: 43690,
            program: vec![4, 0],
        };
        let result = part1(&input);
        assert_eq!(result.rb, 44354);
    }
}
