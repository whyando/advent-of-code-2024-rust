/*

https://adventofcode.com/2024/day/10

part 1:
standard bfs stuff

part 2:
again standard bfs stuff

I was a bit wary of implementing bfs in rust for some reason because graph traversal sounds a bit tricky.
But it was actually really easy. Having no recusion and having the whole graph in one function made it pretty easy.

*/

use std::collections::VecDeque;

fn main() {
    let input = parse(include_str!("../../input/10.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 531);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 1210);
}

type Input = Vec<Vec<i8>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c|
                    // parse c as base 10 digit
                    c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn bfs(grid: &Input, start_i: usize, start_j: usize) -> i64 {
    let mut state = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut num_terminal_nodes = 0;
    queue.push_back((start_i, start_j));
    while let Some((i, j)) = queue.pop_front() {
        // println!("i: {}, j: {}, {}", i, j, grid[i][j]);
        if grid[i][j] == 9 {
            num_terminal_nodes += 1;
        }

        // Consider edges where the v' = v + 1
        for (di, dj) in DIRECTIONS.iter() {
            let i1 = i as i64 + di;
            let j1 = j as i64 + dj;
            if 0 <= i1 && i1 < grid.len() as i64 && 0 <= j1 && j1 < grid[0].len() as i64 {
                let i1 = i1 as usize;
                let j1 = j1 as usize;
                if grid[i1][j1] == grid[i][j] + 1 && state[i1][j1] == 0 {
                    state[i1][j1] = 1;
                    queue.push_back((i1, j1));
                }
            }
        }
    }
    num_terminal_nodes
}

fn bfs_part2(grid: &Input, start_i: usize, start_j: usize) -> i64 {
    let mut state = vec![vec![0; grid[0].len()]; grid.len()];
    let mut count = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut num_routes = 0;
    queue.push_back((start_i, start_j));
    count[start_i][start_j] = 1;
    while let Some((i, j)) = queue.pop_front() {
        // println!("i: {}, j: {}, {} {}", i, j, grid[i][j], count[i][j]);
        if grid[i][j] == 9 {
            num_routes += count[i][j];
        }

        // Consider edges where the v' = v + 1
        for (di, dj) in DIRECTIONS.iter() {
            let i1 = i as i64 + di;
            let j1 = j as i64 + dj;
            if 0 <= i1 && i1 < grid.len() as i64 && 0 <= j1 && j1 < grid[0].len() as i64 {
                let i1 = i1 as usize;
                let j1 = j1 as usize;
                if grid[i1][j1] == grid[i][j] + 1 {
                    count[i1][j1] += count[i][j];
                    if state[i1][j1] == 0 {
                        state[i1][j1] = 1;
                        queue.push_back((i1, j1));
                    }
                }
            }
        }
    }
    num_routes
}

fn part1(input: &Input) -> i64 {
    // Consider each starting point individually
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                let score = bfs(input, i, j);
                // println!("i: {}, j: {}, score: {}", i, j, score);
                sum += score;
            }
        }
    }
    sum
}

fn part2(input: &Input) -> i64 {
    // Consider each starting point individually
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                let score = bfs_part2(input, i, j);
                // println!("i: {}, j: {}, score: {}", i, j, score);
                sum += score;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/10_example.txt"));
        assert_eq!(part1(&input), 1);
        assert_eq!(part2(&input), 16);
    }
    #[test]
    fn test2() {
        let input = parse(include_str!("../../input/10_example2.txt"));
        assert_eq!(part1(&input), 36);
        assert_eq!(part2(&input), 81);
    }
}
