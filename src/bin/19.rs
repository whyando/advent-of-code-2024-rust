/*

https://adventofcode.com/2024/day/19

part1: classic DP
part2: again classic DP

*/

fn main() {
    let input = parse(include_str!("../../input/19.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 236);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 643685981770598);
}

#[derive(Debug)]
struct Input {
    available: Vec<String>,
    designs: Vec<String>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let available = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();
    Input { available, designs }
}

fn part1(input: &Input) -> i64 {
    let mut count = 0;
    for design in &input.designs {
        // let x[i] = 1 if the prefix of design[..i] is makeable
        let mut x = vec![false; design.len() + 1];
        x[0] = true;

        for i in 1..=design.len() {
            for piece in &input.available {
                let len = piece.len();
                if len > i {
                    continue;
                }
                if x[i - piece.len()] {
                    if piece == &design[(i - len)..i] {
                        x[i] = true;
                    }
                }
            }
        }
        if x[design.len()] {
            count += 1;
        }
    }
    count
}

fn part2(input: &Input) -> i64 {
    let mut count = 0;
    for design in &input.designs {
        // let x[i] = 1 if the prefix of design[..i] is makeable
        let mut x = vec![0; design.len() + 1];
        x[0] = 1;

        for i in 1..=design.len() {
            for piece in &input.available {
                let len = piece.len();
                if len > i {
                    continue;
                }
                if piece == &design[(i - len)..i] {
                    x[i] += x[i - piece.len()];
                }
            }
        }
        count += x[design.len()];
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/19_example.txt"));
        assert_eq!(part1(&input), 6);
        assert_eq!(part2(&input), 16);
    }
}
