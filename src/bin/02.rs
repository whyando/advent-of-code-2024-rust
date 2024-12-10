fn main() {
    let input = parse(include_str!("../../input/02.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 314);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 373);
}

type Input = Vec<Vec<i64>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}
fn is_safe(x: &Vec<i64>) -> bool {
    let mut decreasing = true;
    let mut increasing = true;
    for i in 0..x.len() - 1 {
        let diff = (x[i] - x[i + 1]).abs();
        if !(diff >= 1 && diff <= 3) {
            return false;
        }
        if x[i + 1] > x[i] {
            decreasing = false;
        }
        if x[i + 1] < x[i] {
            increasing = false;
        }
    }
    if !(decreasing || increasing) {
        return false;
    }
    true
}

fn is_safe_p2(x: &Vec<i64>) -> bool {
    // for each item in the row, try removing it and see if the row is safe
    for i in 0..x.len() {
        let mut row_copy = x.clone();
        row_copy.remove(i);
        if is_safe(&row_copy) {
            return true;
        }
    }
    false
}

fn part1(input: &Input) -> i64 {
    input.iter().filter(|row| is_safe(row)).count() as i64
}

fn part2(input: &Input) -> i64 {
    input.iter().filter(|row| is_safe_p2(row)).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/02_example.txt"));
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
