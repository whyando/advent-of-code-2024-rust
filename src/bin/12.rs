/*

https://adventofcode.com/2024/day/12

part1:
standard bfs stuff: bfs through the grid where edges are if the value is the same
then the area is the number of vertexes visited and the perimeter contribution is 4 - number of edges for each vertex

part2:
bit more tricky, but the previous solution already found all the edges for a plot, so I just stuck them in a set and then
grouped them if they were part of the same fence by moving left and right along the fence

*/

use std::collections::{BTreeSet, VecDeque};

fn main() {
    let input = parse(include_str!("../../input/12.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1431440);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 869070);
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(grid: &Input) -> i64 {
    let mut state = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut total_price = 0;

    for start_i in 0..grid.len() {
        for start_j in 0..grid[0].len() {
            if state[start_i][start_j] == 1 {
                continue;
            }

            // Run BFS starting from (start_i, start_j)
            let mut area = 0;
            let mut perimeter = 0;
            queue.push_back((start_i, start_j));
            state[start_i][start_j] = 1;
            while let Some((i, j)) = queue.pop_front() {
                area += 1;
        
                // Consider edges where the v' = v + 1
                let mut num_edges = 0;
                for (di, dj) in DIRECTIONS.iter() {
                    let i1 = i as i64 + di;
                    let j1 = j as i64 + dj;
                    if 0 <= i1 && i1 < grid.len() as i64 && 0 <= j1 && j1 < grid[0].len() as i64 {
                        let i1 = i1 as usize;
                        let j1 = j1 as usize;
                        if grid[i1][j1] == grid[i][j] {
                            num_edges += 1;
                            if state[i1][j1] == 0 {
                                state[i1][j1] = 1;
                                queue.push_back((i1, j1));
                            }
                        }
                    }
                }
                perimeter += 4 - num_edges;
            }
            println!("{} ({}, {}) -> area: {}, perimeter: {}", grid[start_i][start_j], start_i, start_j, area, perimeter);
            total_price += area * perimeter;
        }
    }

    total_price
}

fn part2(grid: &Input) -> i64 {
    let mut state = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut total_price = 0;

    for start_i in 0..grid.len() {
        for start_j in 0..grid[0].len() {
            if state[start_i][start_j] == 1 {
                continue;
            }

            // Run BFS starting from (start_i, start_j)
            let mut area = 0;
            let mut fence = BTreeSet::new();
            queue.push_back((start_i, start_j));
            state[start_i][start_j] = 1;
            while let Some((i, j)) = queue.pop_front() {
                area += 1;
        
                // Consider edges where the v' = v + 1
                for (dir, (di, dj)) in DIRECTIONS.iter().enumerate() {
                    let mut is_edge = false;
                    let i1 = i as i64 + di;
                    let j1 = j as i64 + dj;
                    if 0 <= i1 && i1 < grid.len() as i64 && 0 <= j1 && j1 < grid[0].len() as i64 {
                        let i1 = i1 as usize;
                        let j1 = j1 as usize;
                        if grid[i1][j1] == grid[i][j] {
                            is_edge = true;
                            if state[i1][j1] == 0 {
                                state[i1][j1] = 1;
                                queue.push_back((i1, j1));
                            }
                        }
                    }

                    if !is_edge {
                        // then this is a fence
                        fence.insert((i as i64 , j as i64, dir));
                    }
                }
            }
            println!("{:?}", fence);
            
            // Now, for each piece of fence, move left and right to see if those are also fences
            let mut checked = BTreeSet::new();
            let mut fence_length = 0;
            for (i, j, dir) in fence.iter() {
                if checked.contains(&(*i, *j, dir)) {
                    continue;
                }
                fence_length += 1;
                let left_dir = (dir + 3) % 4;
                let right_dir = (dir + 1) % 4;
                
                // LEFT
                for k in 1..{
                    let mut is_also_fence = false;
                    let i1 = i + k * DIRECTIONS[left_dir as usize].0;
                    let j1 = j + k * DIRECTIONS[left_dir as usize].1;
                    if 0 <= i1 && i1 < grid.len() as i64 && 0 <= j1 && j1 < grid[0].len() as i64 {
                        if fence.contains(&(i1 as i64, j1 as i64, *dir)) {
                            is_also_fence = true;
                            checked.insert((i1 as i64, j1 as i64, dir));
                        }
                    }
                    if !is_also_fence {
                        break;
                    }
                }
                // RIGHT
                for k in 1..{
                    let mut is_also_fence = false;
                    let i1 = i + k * DIRECTIONS[right_dir as usize].0;
                    let j1 = j + k * DIRECTIONS[right_dir as usize].1;
                    if 0 <= i1 && i1 < grid.len() as i64 && 0 <= j1 && j1 < grid[0].len() as i64 {
                        if fence.contains(&(i1, j1, *dir)) {
                            is_also_fence = true;
                            checked.insert((i1, j1, dir));
                        }
                    }
                    if !is_also_fence {
                        break;
                    }
                }
            }

            println!("{} ({}, {}) -> area: {}, fence_length: {}", grid[start_i][start_j], start_i, start_j, area, fence_length);
            total_price += area * fence_length;
        }
    }

    total_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/12_example.txt"));
        assert_eq!(part1(&input), 1930);
        assert_eq!(part2(&input), 1206);
    }
}
