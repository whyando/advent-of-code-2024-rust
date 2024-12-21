/*

https://adventofcode.com/2024/day/18

part1: easy
simply get the maze and then find the distance.

part2: easy
I thought this was going to have the maze update with new obstacles at each tick which is why I coded part1 the way I did.
But actually it was just checking whether the exit was reachable after each new obstacle was added. Which is trivial to run
a dfs after each new obstacle is added.

*/

fn main() {
    let input = parse(include_str!("../../input/18.txt"));

    let part1 = part1(&input, 71, 1024);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 310);

    let part2 = part2(&input, 71);
    println!("Part 2: {}", part2);
    assert_eq!(part2, "16,46");
}

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

type Input = Vec<Coord>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Coord { x, y }
        })
        .collect()
}

// fn print_grid(grid: &Vec<Vec<i64>>) {
//     for row in grid {
//         for cell in row {
//             match cell {
//                 0 => print!("."),
//                 1 => print!("O"),
//                 2 => print!("#"),
//                 _ => panic!("Unknown cell value"),
//             }
//         }
//         println!();
//     }
// }

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &Input, size: usize, skipped_bytes: i64) -> i64 {
    let mut grid = vec![vec![0; size]; size];
    for t in 0..skipped_bytes {
        let coord = &input[t as usize];
        grid[coord.y as usize][coord.x as usize] = 2;
    }
    // print_grid(&grid);

    // let U[t] be the set of points reachable from the origin within t steps
    grid[0][0] = 1;
    for t in 1.. {
        let mut grid1 = grid.clone();
        for x in 0..size {
            for y in 0..size {
                if grid[y][x] == 0 {
                    let mut reachable = false;
                    for (dx, dy) in DIRECTIONS.iter() {
                        let x1 = x as i64 + dx;
                        let y1 = y as i64 + dy;
                        if x1 >= 0 && x1 < size as i64 && y1 >= 0 && y1 < size as i64 {
                            if grid[y1 as usize][x1 as usize] == 1 {
                                reachable = true;
                                break;
                            }
                        }
                    }
                    if reachable {
                        grid1[y][x] = 1;
                    }
                }
            }
        }
        grid = grid1;
        // println!("t = {}", t);
        // print_grid(&grid);
        if grid[size - 1][size - 1] == 1 {
            return t;
        }
    }
    0
}

fn path_exists(grid: &Vec<Vec<i64>>) -> bool {
    let n = grid.len() as i64;
    // dfs
    let mut stack = vec![(0i64, 0i64)];
    let mut visited = vec![vec![false; n as usize]; n as usize];

    while stack.len() > 0 {
        let (x, y) = stack.pop().unwrap();
        // println!("x = {}, y = {}", x, y);
        if visited[y as usize][x as usize] {
            continue;
        }
        visited[y as usize][x as usize] = true;
        if x == n - 1 && y == n - 1 {
            return true;
        }
        for (dx, dy) in DIRECTIONS.iter() {
            let x1 = x as i64 + dx;
            let y1 = y as i64 + dy;
            if x1 >= 0 && x1 < n as i64 && y1 >= 0 && y1 < n {
                if grid[y1 as usize][x1 as usize] == 0 {
                    stack.push((x1, y1));
                }
            }
        }
    }
    false
}

fn part2(input: &Input, size: usize) -> String {
    let mut grid = vec![vec![0; size]; size];
    for t in 0.. {
        let coord = &input[t as usize];
        grid[coord.y as usize][coord.x as usize] = 2;
        // check if exit is reachable
        if !path_exists(&grid) {
            return format!("{},{}", coord.x, coord.y);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/18_example.txt"));
        assert_eq!(part1(&input, 7, 12), 22);
        assert_eq!(part2(&input, 7), "6,1");
    }
}
