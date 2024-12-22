use std::collections::BTreeMap;

fn main() {
    let input = parse(include_str!("../../input/22.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 16894083306);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 1925);
}

type Input = Vec<i64>;

const M: i64 = 16777216;

fn next(mut x: i64) -> i64 {
    x ^= x << 6;
    x &= M - 1;
    x ^= x >> 5;
    x &= M - 1;
    x ^= x << 11;
    x &= M - 1;
    x
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for &x in input {
        let mut x = x;
        for _ in 0..2000 {
            x = next(x);
        }
        sum += x;
    }
    sum
}

fn part2(input: &Input) -> i64 {
    let mut changes = vec![vec![0; 2000]; input.len()];
    let mut first: Vec<BTreeMap<(i64, i64, i64, i64), i64>> = vec![BTreeMap::new(); input.len()];
    let mut values: Vec<Vec<i64>> = vec![vec![0; 2000]; input.len()];
    for i in 0..input.len() {
        let mut x = input[i];
        for j in 0..2000 {
            let next = next(x);
            let change = (next % 10) - (x % 10);
            changes[i][j] = change;
            values[i][j] = next;
            x = next;

            if j >= 3 {
                let seq = (changes[i][j-3], changes[i][j-2], changes[i][j-1], changes[i][j]);
                if !first[i].contains_key(&seq) {                    
                    first[i].insert(seq.clone(), j as i64);
                }
            }
        }
    }

    // now we can test each candidate sequence
    
    let mut best = 0;
    let mut best_seq = (0,0,0,0);
    for a in -9..=9 {
        for b in -9..=9 {
            if a + b < -9 || a + b > 9 {
                continue;
            }
            for c in -9..=9 {
                if a + b + c < -9 || a + b + c > 9 {
                    continue;
                }
                for d in -9..=9 {
                    if a + b + c + d < -9 || a + b + c + d > 9 {
                        continue;
                    }


                    let seq = (a,b,c,d);
                    let mut sum = 0;
                    for i in 0..input.len() {
                        if let Some(j) = first[i].get(&seq) {
                            sum += values[i][*j as usize] % 10;
                        }
                    }

                    if sum > best {
                        best = sum;
                        best_seq = seq;
                    }
                }
            }
        }
    }
    println!("Best: {} @ {:?}", best, best_seq);
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/22_example.txt"));
        assert_eq!(part1(&input), 37327623);
    }

    #[test]
    fn test2() {
        let input = vec![1, 2, 3, 2024];
        assert_eq!(part2(&input), 23);
    }
}
