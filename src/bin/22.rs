
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
    let seq_index_max = 19 * 19 * 19 * 19;
    let mut seq_sum =  vec![0; seq_index_max];
    for i in 0..input.len() {
        let mut seen = vec![false; seq_index_max];
        let mut x = input[i];
        let mut seq = (0, 0, 0, 0);
        for j in 0..2000 {
            let next = next(x);
            let change = ((next % 10) - (x % 10) + 9) as usize; // 0 - 18
            seq.3 = seq.2;
            seq.2 = seq.1;
            seq.1 = seq.0;
            seq.0 = change;
            let seq_index = 19 * 19 * 19 * seq.3 + 19 * 19 * seq.2 + 19 * seq.1 + seq.0;
            x = next;

            if j >= 3 {
                if !seen[seq_index] {
                    seen[seq_index] = true;
                    let value = x % 10;
                    seq_sum[seq_index] += value;
                }
            }
        }
    }
    
    let mut best = 0;
    let mut best_seq = (0, 0, 0, 0);
    for seq_index in 0..seq_index_max {
        if seq_sum[seq_index] > best {
            best = seq_sum[seq_index];
            best_seq = (
                (seq_index / (19 * 19 * 19)) as i64 - 9,
                ((seq_index / (19 * 19)) % 19) as i64 - 9,
                ((seq_index / 19) % 19) as i64 - 9,
                (seq_index % 19) as i64 - 9,
            );
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
