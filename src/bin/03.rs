fn main() {
    let input = parse(include_str!("../../input/03.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 159892596);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 92626942);
}

type Input = String;

fn parse(input: &str) -> Input {
    input.trim().to_owned()
}

fn part1(input: &Input) -> i64 {
    let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for m in regex.captures_iter(input) {
        let a = m[1].parse::<i64>().unwrap();
        let b = m[2].parse::<i64>().unwrap();
        println!("{} * {} = {}", a, b, a * b);
        sum += a * b;
    }
    sum
}

fn part2(input: &Input) -> i64 {
    let input_rev = input.chars().rev().collect::<String>();
    println!("input_rev: {:?}", input_rev);
    let n = input.len();

    let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = regex::Regex::new(r"\)\(od").unwrap();
    let dont_regex = regex::Regex::new(r"\)\(t'nod").unwrap();

    let mut sum = 0;
    for m in mul_regex.captures_iter(input) {
        let index = m.get(0).unwrap().start();
        let a = m[1].parse::<i64>().unwrap();
        let b = m[2].parse::<i64>().unwrap();
        // look backwards for do

        // search 0..index for do
        // which is N-index..N in the reversed string
        let prev_do = do_regex.find(&input_rev[n - index..n]);
        let prev_dont = dont_regex.find(&input_rev[n - index..n]);
        println!("prev_do: {:?}, prev_dont: {:?}", prev_do, prev_dont);

        // Check which one is closer
        let mut include = true;
        if let Some(prev_dont) = prev_dont {
            if let Some(prev_do) = prev_do {
                // Only include if 'do' index is lower
                include = prev_do.start() < prev_dont.start();
            } else {
                include = false;
            }
        }
        if include {
            println!("{} * {} = {}", a, b, a * b);
            sum += a * b;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/03_example.txt"));
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = parse(include_str!("../../input/03_example2.txt"));
        assert_eq!(part2(&input), 48);
    }
}
