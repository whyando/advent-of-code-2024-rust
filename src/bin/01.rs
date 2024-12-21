/*

https://adventofcode.com/2024/day/1

part 1:
simple calculation

part 2:
again simple calculation

*/

use std::collections::BTreeMap;

fn main() {
    let input = parse(include_str!("../../input/01.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1506483);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 23126924);
}

struct Input {
    list1: Vec<i64>,
    list2: Vec<i64>,
}

fn parse(input: &str) -> Input {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i64>().unwrap());
        list2.push(iter.next().unwrap().parse::<i64>().unwrap());
    }
    Input { list1, list2 }
}

fn part1(input: &Input) -> i64 {
    let mut list1 = input.list1.clone();
    let mut list2 = input.list2.clone();

    // sort both lists
    list1.sort();
    list2.sort();

    // sum of absolute differences
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &Input) -> i64 {
    let mut score = 0;
    let mut map1 = BTreeMap::<i64, i64>::new();
    for i in &input.list1 {
        let e = map1.entry(*i).or_insert(0);
        *e += 1;
    }
    let mut map2 = BTreeMap::<i64, i64>::new();
    for i in &input.list2 {
        let e = map2.entry(*i).or_insert(0);
        *e += 1;
    }
    for (k, v) in map1.iter() {
        if let Some(v2) = map2.get(k) {
            score += k * v * v2;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/01_example.txt"));
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 31);
    }
}
