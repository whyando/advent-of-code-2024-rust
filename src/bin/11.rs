/*

https://adventofcode.com/2024/day/11


part 1:
seemed within the realm of computation because after 25 splits, the maximum number of stones is 2^25 approx 30M

but I added a memoization cache just in case

part 2:
definitely not within brute force terrority but the memoization took care of it

It seems like the stones never actually get that large. Over 18 digits would have created overflow issues.
But if most of the stones are 7 digits or less, than that's only 10^7 * 75 values to memoize

*/

use std::{collections::HashMap, sync::Mutex};

fn main() {
    let input = parse(include_str!("../../input/11.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 186996);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 221683913164898);
}

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    input
    .split(' ')
    .map(|line| line.parse().unwrap())
    .collect()
}

lazy_static::lazy_static!(
    static ref F_CACHE: Mutex<HashMap<(i64, i64), i64>> = Mutex::new(HashMap::new());
);

fn f_inner(n: i64, t: i64) -> i64 {
    if t == 0 {
        return 1;
    }
    if n == 0 {
        return f(1, t - 1);
    }
    let n_string = n.to_string();
    if n_string.len() % 2 == 0 {
        let half = n_string.len() / 2;
        let left = n_string[..half].parse().unwrap();
        let right = n_string[half..].parse().unwrap();
        return f(left, t - 1) + f(right, t - 1);
    }
    // watch out for overflow here
    f(n * 2024, t - 1)
}

// Number of stones after t ticks
fn f(n: i64, t: i64) -> i64 {
    {
        let cache = F_CACHE.lock().unwrap();
        if let Some(&result) = cache.get(&(n, t)) {
            return result;
        }
    }
    println!("f({}, {})", n, t);
    let result = f_inner(n, t);
    println!("f({}, {}) = {}", n, t, result);
    {
        let mut cache = F_CACHE.lock().unwrap();
        cache.insert((n, t), result);
    }
    result    
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for x in input {
        sum += f(*x, 25);
    }
    sum
}

fn part2(input: &Input) -> i64 {
    let mut sum = 0;
    for x in input {
        sum += f(*x, 75);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/11_example.txt"));
        assert_eq!(part1(&input), 55312);
        assert_eq!(part2(&input), 65601038650482); // not confirmed
    }
}
