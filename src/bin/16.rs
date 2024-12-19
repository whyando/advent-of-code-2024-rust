/*

https://adventofcode.com/2024/day/16

part1:
straightforward dijkstra. vertices are (i, j, dir) where i, j are the current position and dir is the direction we are
facing. Edges are moving forward, turning left or right. The cost of moving forward is 1, turning left or right is 1000.

part2:
Easiest way to solve this is to run a forward dijkstra from the start point and a backwards dijkstra from the end point.
Then, for each cell, we check if there is a path that goes through that cell. If there is, we increment the count.

*/

use std::collections::BinaryHeap;

fn main() {
    let input = parse(include_str!("../../input/16.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 135512);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 541);
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn get_pos(input: &Input, c: char) -> (i64, i64) {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == c {
                return (i as i64, j as i64);
            }
        }
    }
    panic!("Not found");
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &Input) -> i64 {
    // dijkstra
    let (start_i, start_j) = get_pos(input, 'S');
    let start_dir = 1;

    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![vec![std::i64::MAX; 4]; input[0].len()]; input.len()];
    let mut visited = vec![vec![vec![false; 4]; input[0].len()]; input.len()];
    heap.push((0, start_i, start_j, start_dir));

    loop {
        let (cost, i, j, dir) = heap.pop().unwrap();
        if visited[i as usize][j as usize][dir as usize] {
            continue;
        }
        visited[i as usize][j as usize][dir as usize] = true;
        dist[i as usize][j as usize][dir as usize] = cost;

        if input[i as usize][j as usize] == 'E' {
            return -cost;
        }

        // edges: forward
        let i1 = i + DIRECTIONS[dir as usize].0;
        let j1 = j + DIRECTIONS[dir as usize].1;
        if i1 >= 0 && i1 < input.len() as i64 && j1 >= 0 && j1 < input[0].len() as i64 {
            if input[i1 as usize][j1 as usize] != '#' {
                heap.push((cost - 1, i1, j1, dir));
            }
        }
        // edges: turn left or right
        let left_dir = (dir + 3) % 4;
        let right_dir = (dir + 1) % 4;
        heap.push((cost - 1000, i, j, left_dir));
        heap.push((cost - 1000, i, j, right_dir));
    }
}

fn part2(input: &Input) -> i64 {
    // Forward dijkstra
    let dist_from_start = {
        let (start_i, start_j) = get_pos(input, 'S');
        let (end_i, end_j) = get_pos(input, 'E');
        let start_dir = 1;
        let mut heap = BinaryHeap::new();
        let mut dist = vec![vec![vec![std::i64::MAX; 4]; input[0].len()]; input.len()];
        let mut visited = vec![vec![vec![false; 4]; input[0].len()]; input.len()];
        heap.push((0, start_i, start_j, start_dir));

        while heap.len() > 0 {
            let (cost, i, j, dir) = heap.pop().unwrap();
            if visited[i as usize][j as usize][dir as usize] {
                continue;
            }
            visited[i as usize][j as usize][dir as usize] = true;
            dist[i as usize][j as usize][dir as usize] = cost;

            // edges: forward
            let i1 = i + DIRECTIONS[dir as usize].0;
            let j1 = j + DIRECTIONS[dir as usize].1;
            if i1 >= 0 && i1 < input.len() as i64 && j1 >= 0 && j1 < input[0].len() as i64 {
                if input[i1 as usize][j1 as usize] != '#' {
                    heap.push((cost - 1, i1, j1, dir));
                }
            }
            // edges: turn left or right
            let left_dir = (dir + 3) % 4;
            let right_dir = (dir + 1) % 4;
            let is_end_point = (i, j) == (end_i, end_j);
            let rotate_cost = if is_end_point { 0 } else { 1000 };
            heap.push((cost - rotate_cost, i, j, left_dir));
            heap.push((cost - rotate_cost, i, j, right_dir));
        }
        dist
    };

    // Backwards dijkstra
    let dist_from_end = {
        let (start_i, start_j) = get_pos(input, 'E');
        let start_dir = 1;
        let mut heap = BinaryHeap::new();
        let mut dist = vec![vec![vec![std::i64::MAX; 4]; input[0].len()]; input.len()];
        let mut visited = vec![vec![vec![false; 4]; input[0].len()]; input.len()];
        heap.push((0, start_i, start_j, start_dir));

        while heap.len() > 0 {
            let (cost, i, j, dir) = heap.pop().unwrap();
            if visited[i as usize][j as usize][dir as usize] {
                continue;
            }
            visited[i as usize][j as usize][dir as usize] = true;
            dist[i as usize][j as usize][dir as usize] = cost;

            // edges: forward (backwards)
            let i1 = i - DIRECTIONS[dir as usize].0;
            let j1 = j - DIRECTIONS[dir as usize].1;
            if i1 >= 0 && i1 < input.len() as i64 && j1 >= 0 && j1 < input[0].len() as i64 {
                if input[i1 as usize][j1 as usize] != '#' {
                    heap.push((cost - 1, i1, j1, dir));
                }
            }
            // edges: turn left or right
            let left_dir = (dir + 3) % 4;
            let right_dir = (dir + 1) % 4;
            let is_end_point = (i, j) == (start_i, start_j);
            let rotate_cost = if is_end_point { 0 } else { 1000 };
            heap.push((cost - rotate_cost, i, j, left_dir));
            heap.push((cost - rotate_cost, i, j, right_dir));
        }
        dist
    };
    let (start_i, start_j) = get_pos(input, 'S');
    let (end_i, end_j) = get_pos(input, 'E');
    let distance = dist_from_start[end_i as usize][end_j as usize][0];
    println!("Distance: {}", distance);
    assert_eq!(
        distance,
        dist_from_end[start_i as usize][start_j as usize][1]
    );

    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let mut best_path = false;
            for dir in 0..4 {
                if dist_from_start[i][j][dir] == std::i64::MAX
                    || dist_from_end[i][j][dir] == std::i64::MAX
                {
                    continue;
                }
                if dist_from_start[i][j][dir] + dist_from_end[i][j][dir] == distance {
                    best_path = true;
                }
            }
            if best_path {
                count += 1;
            }
            if best_path {
                print!("O");
            } else {
                print!("{}", input[i][j]);
            }
        }
        println!();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/16_example.txt"));
        assert_eq!(part1(&input), 7036);
        assert_eq!(part2(&input), 45);
    }

    #[test]
    fn test2() {
        let input = parse(include_str!("../../input/16_example2.txt"));
        assert_eq!(part1(&input), 11048);
        assert_eq!(part2(&input), 64);
    }
}
