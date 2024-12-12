/*

https://adventofcode.com/2024/day/6

part 1.
trivial simulation - O(N^2) steps

part 2.
there's O(N^2) possible potential obstacle placements, so the easy solution is to just try them all.
O(N^4) is fine for N=130

we could have cut the search by noting that the obstacle must be in the original path.
If I had to speed up the simulation I would try precomputing next[i][j][dir] = (i1, j1, dir1) as the next position
reached by moving in a straight line

*/

use std::char;

fn main() {
    let input = parse(include_str!("../../input/06.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 5318);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 1831);
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

const DIRECTIONS: [(i64, i64); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

fn part1(input: &Input) -> i64 {
    let mut x = input.clone();
    println!("{:?}", input);
    // find the coords of the ^ character
    let mut cur_i = -1;
    let mut cur_j = -1;
    let mut cur_dir = 0;

    for i in 0..x.len() {
        for j in 0..x[i].len() {
            if x[i][j] == '^' {
                cur_i = i as i64;
                cur_j = j as i64;
            }
        }
    }
    assert_ne!(cur_i, -1);
    assert_ne!(cur_j, -1);
    x[cur_i as usize][cur_j as usize] = 'X';

    loop {
        println!("i: {}, j: {}, dir: {}", cur_i, cur_j, cur_dir);
        let i1 = cur_i + DIRECTIONS[cur_dir].0;
        let j1 = cur_j + DIRECTIONS[cur_dir].1;

        if !(0 <= i1 && i1 < x.len() as i64 && 0 <= j1 && j1 < x[0].len() as i64) {
            // out of bounds
            break;
        }
        let next = x[i1 as usize][j1 as usize];
        if next == '#' {
            // turn right
            cur_dir = (cur_dir + 1) % 4;
        } else {
            // move forward
            cur_i = i1;
            cur_j = j1;
            x[cur_i as usize][cur_j as usize] = 'X';
        }
    }

    // print the map
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            print!("{}", x[i][j]);
        }
        println!();
    }

    // count 'X's
    let count = x.iter().map(|row| row.iter().filter(|&&c| c == 'X').count()).sum::<usize>();
    count as i64
}

fn is_trapped(x: &Input) -> bool {
    // find the coords of the ^ character
    let mut cur_i = -1;
    let mut cur_j = -1;
    let mut cur_dir = 0;

    let mut seen: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; x[0].len()]; x.len()];

    for i in 0..x.len() {
        for j in 0..x[i].len() {
            if x[i][j] == '^' {
                cur_i = i as i64;
                cur_j = j as i64;
            }
        }
    }
    assert_ne!(cur_i, -1);
    assert_ne!(cur_j, -1);
    seen[cur_i as usize][cur_j as usize][cur_dir] = true;

    loop {
        // println!("i: {}, j: {}, dir: {}", cur_i, cur_j, cur_dir);
        let i1 = cur_i + DIRECTIONS[cur_dir].0;
        let j1 = cur_j + DIRECTIONS[cur_dir].1;

        if !(0 <= i1 && i1 < x.len() as i64 && 0 <= j1 && j1 < x[0].len() as i64) {
            // out of bounds
            return false;
        }
        let next = x[i1 as usize][j1 as usize];
        if next == '#' {
            // turn right
            cur_dir = (cur_dir + 1) % 4;
        } else {
            // move forward
            cur_i = i1;
            cur_j = j1;
        }
        if seen[cur_i as usize][cur_j as usize][cur_dir] {
            return true;
        }
        seen[cur_i as usize][cur_j as usize][cur_dir] = true;
    }
}

fn part2(input: &Input) -> i64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '.' {
                let mut x = input.clone();
                x[i][j] = '#';
                if is_trapped(&x) {
                    count += 1;
                }
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
        let input = parse(include_str!("../../input/06_example.txt"));
        assert_eq!(part1(&input), 41);
        assert_eq!(part2(&input), 6);
    }
}
