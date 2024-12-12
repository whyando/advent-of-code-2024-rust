/*

https://adventofcode.com/2024/day/8

part1:

trivial to iterate the pairs of antenna and calculate the two antinode positions with vector addition.
I guess the trap is if you started from a coord, and asked yourself if it was an antinode.

part2:

still trivial to calculate the extra antinode positions

*/

use std::collections::BTreeMap;

fn main() {
    let input = parse(include_str!("../../input/08.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 379);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 0);
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

struct Coord {
    i: i64,
    j: i64,
}

fn part1(input: &Input) -> i64 {
    let mut hit = vec![vec![false; input[0].len()]; input.len()];

    // Start by listing the antenna
    let mut antenna = BTreeMap::<char, Vec<Coord>>::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let c = input[i][j];
            if c != '.' {
                antenna.entry(c).or_default().push(Coord { i: i as i64, j: j as i64 });
            }
        }
    }

    for (_c, coords) in antenna.iter() {
        // For each pair of antennaa, check their 2 antinodes
        let n = coords.len();
        for a in 0..n {
            for b in (a+1)..n {
                let di = coords[b].i - coords[a].i;
                let dj = coords[b].j - coords[a].j;

                for k in [-1, 2] {
                    let i1 = coords[a].i + di*k;
                    let j1 = coords[a].j + dj*k;
                    if 0 <= i1 && i1 < input.len() as i64 && 0 <= j1 && j1 < input[0].len() as i64 {
                        hit[i1 as usize][j1 as usize] = true;
                    }
                }
            }
        }
    }

    // count hit
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if hit[i][j] {
                count += 1;
            }
        }
    }
    count as i64
}

fn part2(input: &Input) -> i64 {
    let mut hit = vec![vec![false; input[0].len()]; input.len()];

    // Start by listing the antenna
    let mut antenna = BTreeMap::<char, Vec<Coord>>::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let c = input[i][j];
            if c != '.' {
                antenna.entry(c).or_default().push(Coord { i: i as i64, j: j as i64 });
            }
        }
    }

    for (_c, coords) in antenna.iter() {
        // For each pair of antennaa, check their 2 antinodes
        let n = coords.len();
        for a in 0..n {
            for b in (a+1)..n {
                let di = coords[b].i - coords[a].i;
                let dj = coords[b].j - coords[a].j;
   
                for k in 0.. {
                    let i1 = coords[a].i + di*k;
                    let j1 = coords[a].j + dj*k;
                    if 0 <= i1 && i1 < input.len() as i64 && 0 <= j1 && j1 < input[0].len() as i64 {
                        hit[i1 as usize][j1 as usize] = true;
                    } else {
                        break;
                    }
                }  
                for k in 1.. {
                    let i1 = coords[a].i - di*k;
                    let j1 = coords[a].j - dj*k;
                    if 0 <= i1 && i1 < input.len() as i64 && 0 <= j1 && j1 < input[0].len() as i64 {
                        hit[i1 as usize][j1 as usize] = true;
                    } else {
                        break;
                    }
                }              
            }
        }
    }

    // count hit
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if hit[i][j] {
                count += 1;
            }
        }
    }
    count as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/08_example.txt"));
        assert_eq!(part1(&input), 14);
        assert_eq!(part2(&input), 34);
    }
}
