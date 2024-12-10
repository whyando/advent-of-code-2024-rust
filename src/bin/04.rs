fn main() {
    let input = parse(include_str!("../../input/04.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 2633);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 1936);
}

type Input = Vec<String>;

fn parse(input: &str) -> Input {
    let x: Vec<String> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    // make sure all lines have the same length
    let m = x[0].len();
    assert!(x.iter().all(|line| line.len() == m));
    assert_eq!(m, x.len());
    x
}

const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const DIRECTIONS_DIAG: [(i64, i64); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

fn check_match(x: &Input, i: i64, j: i64, di: i64, dj: i64) -> bool {
    let target = "XMAS";
    for k in 0..target.len() as i64 {
        let i1 = i + k * di;
        let j1 = j + k * dj;
        if i1 < 0 || i1 >= x.len() as i64 || j1 < 0 || j1 >= x[0].len() as i64 {
            return false;
        }
        if x[i1 as usize].chars().nth(j1 as usize).unwrap()
            != target.chars().nth(k as usize).unwrap()
        {
            return false;
        }
    }
    true
}

fn part1(input: &Input) -> i64 {
    let n = input.len() as i64;
    let m = input[0].len() as i64;
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            for d in DIRECTIONS {
                if check_match(&input, i, j, d.0, d.1) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_match_part2(x: &Input, i: i64, j: i64) -> bool {
    if x[i as usize].chars().nth(j as usize).unwrap() != 'A' {
        return false;
    }

    let mut num_s = 0;
    let mut num_m = 0;

    let mut chars = Vec::<char>::new();
    for d in DIRECTIONS_DIAG {
        let i1 = i + d.0;
        let j1 = j + d.1;
        let char = x[i1 as usize].chars().nth(j1 as usize).unwrap();
        chars.push(char);
        if char == 'S' {
            num_s += 1;
        } else if char == 'M' {
            num_m += 1;
        }
    }
    if !(num_s == 2 && num_m == 2) {
        return false;
    }
    // make sure it's not opposite:
    if chars[0] == chars[2] && chars[1] == chars[3] {
        return false;
    }
    true
}

fn part2(input: &Input) -> i64 {
    let n = input.len() as i64;
    let m = input[0].len() as i64;
    let mut count = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if check_match_part2(&input, i, j) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/04_example.txt"));
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 9);
    }
}
