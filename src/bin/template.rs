fn main() {
    let input = parse(include_str!("../../input/01.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 0);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 0);
}

type Input = ();

fn parse(input: &str) -> Input {
    ()
}

fn part1(input: &Input) -> i64 {
    0
}

fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/01_example.txt"));
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
