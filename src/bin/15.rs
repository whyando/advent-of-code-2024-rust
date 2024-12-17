/*

https://adventofcode.com/2024/day/15

part1: pretty simple, each movement is just pushing some K boxes in a row. Finding k is just finding the first empty
space

part2: My questionable approach was to keep a queue of boxes to move. Turned out to be quite bug prone and hard to debug.
Main issue was when a box moves, at what point to mark the cell it used to be in as empty. My main bug was marking the cell as
empty when in fact another box had already moved into it. Hard to reason about which order the boxes were moving in.

If I were to do this again, I would iterate strictly row by row and compute the full list of coords to move in each row
then go onto the next. And then actually move the boxes all at the end (Or maybe row by row).
*/

use std::collections::VecDeque;

fn main() {
    let input = parse(include_str!("../../input/15.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1478649);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 1495455);
}

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<char>>,
    moves: Vec<char>,
}

fn parse(input: &str) -> Input {
    // split on double newline
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    // parse first part as grid
    let grid = parts[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    // parse second part as moves
    let moves = parts[1]
        .trim()
        .chars()
        .filter(|c| *c != '\n')
        .collect();
    Input { grid, moves }
}

fn simple_push(grid: &mut Vec<Vec<char>>, i: &mut i64, j: &mut i64, di: i64, dj: i64) {
    // Look ahead until we find a wall or edge of the grid
    for k in 1.. {
        let i1 = *i + di * k;
        let j1 = *j + dj * k;
        if i1 < 0 || i1 >= grid.len() as i64 || j1 < 0 || j1 >= grid[0].len() as i64 {
            // hit edge of grid - nothing happens
            break;
        }
        let cell = grid[i1 as usize][j1 as usize];
        match cell {
            '#' => break, // wall - nothing happens
            '.' => {
                // empty cell - stop and shift cells
                for k1 in (1..=k).rev() {
                    grid[(*i + di * k1) as usize][(*j + dj * k1) as usize] = grid[(*i + di * (k1-1)) as usize][(*j + dj * (k1-1)) as usize];
                }
                grid[*i as usize][*j as usize] = '.';
                *i += di;
                *j += dj;
                break
            }
            'O'|'['|']' => {}, // box - continue
            _ => panic!("invalid cell"),
        }
    }
}

fn part1(input: &Input) -> i64 {
    let mut grid = input.grid.clone();

    let mut i = -1;
    let mut j = -1;
    for (i1, row) in grid.iter().enumerate() {
        for (j1, cell) in row.iter().enumerate() {
            if *cell == '@' {
                i = i1 as i64;
                j = j1 as i64;
                break;
            }
        }
    }
    assert_ne!(i, -1);
    assert_ne!(j, -1);

    for m in &input.moves {
        // println!("{:?}", m);
        let (di, dj) = match m {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("invalid move"),
        };

        // Look ahead until we find a wall or edge of the grid
        simple_push(&mut grid, &mut i, &mut j, di, dj);
    }

    // print grid
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                sum += i*100 + j;
            }
        }
    }
    sum as i64
}

fn part2(input: &Input) -> i64 {
    // Start by scaling up grid
    let mut grid = vec![vec![' '; input.grid[0].len()*2]; input.grid.len()];
    for i in 0..input.grid.len() {
        for j in 0..input.grid[0].len() {
            let new = match input.grid[i][j] {
                '#' => "##",
                '.' => "..",
                'O' => "[]",
                '@' => "@.",
                _ => panic!("invalid cell"),
            };
            grid[i][2*j] = new.chars().nth(0).unwrap();
            grid[i][2*j+1] = new.chars().nth(1).unwrap();
        }
    }

    for m in &input.moves {
        // print_grid(&grid);
        // println!("{:?}", m);
        let (di, dj) = match m {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("invalid move"),
        };

        let mut i = -1;
        let mut j = -1;
        for (i1, row) in grid.iter().enumerate() {
            for (j1, cell) in row.iter().enumerate() {
                if *cell == '@' {
                    i = i1 as i64;
                    j = j1 as i64;
                    break;
                }
            }
        }
        assert_ne!(i, -1);
        assert_ne!(j, -1);        

        if m == &'<' || m == &'>' {
            // Look ahead until we find a wall or edge of the grid
            simple_push(&mut grid, &mut i, &mut j, di, dj);
            continue;
        }

        // Let's just assume that the push suceeds, and revert it if it doesn't
        let mut grid1 = grid.clone();        
    
        // queue of cells to move
        let mut queue = VecDeque::new();
        let mut pushed = vec![vec![false; grid[0].len()]; grid.len()];
        let mut is_failed = false;
        grid1[i as usize][j as usize] = '.';
        queue.push_back((i+di, j+dj, '@'));

        while !queue.is_empty() {
            let (i, j, new_content) = queue.pop_front().unwrap();
            // println!("{:?} {:?} {:?}", i, j, new_content);
            if i < 0 || i >= grid.len() as i64 || j < 0 || j >= grid[0].len() as i64 {
                // hit edge of grid - failed
                is_failed = true;
                break;
            }
            let cell = grid[i as usize][j as usize];
            match cell {
                '#' => {
                    // failed
                    is_failed = true;
                    break;
                }
                '.' => {
                    // empty cell
                    grid1[i as usize][j as usize] = new_content;
                }
                '[' => {
                    // then g[i][j] is a box and also g[i][j+1] is a box
                    assert_eq!(grid[i as usize][j as usize+1], ']');
                    let left_pushed = pushed[(i+di) as usize][(j+dj) as usize];
                    let right_pushed = pushed[(i+di) as usize][(j+dj+1) as usize];
                    assert_eq!(left_pushed, right_pushed);
                    if !left_pushed {
                        queue.push_back((i+di, j+dj, '['));
                        queue.push_back((i+di, j+dj+1, ']'));
                        if !pushed[i as usize][j as usize+1] {
                            grid1[i as usize][j as usize+1] = '.';
                        }
                        pushed[(i+di) as usize][(j+dj) as usize] = true;
                        pushed[(i+di) as usize][(j+dj+1) as usize] = true;
                    }
                    grid1[i as usize][j as usize] = new_content;
                }
                ']'=> {
                    // then g[i][j] is a box and also g[i][j-1] is a box
                    assert_eq!(grid[i as usize][j as usize-1], '[');
                    let left_pushed = pushed[(i+di) as usize][(j+dj-1) as usize];
                    let right_pushed = pushed[(i+di) as usize][(j+dj) as usize];
                    assert_eq!(left_pushed, right_pushed);
                    if !left_pushed {
                        queue.push_back((i+di, j+dj, ']'));
                        queue.push_back((i+di, j+dj-1, '['));
                        if !pushed[i as usize][j as usize-1] {
                            grid1[i as usize][j as usize-1] = '.';
                        }
                        pushed[(i+di) as usize][(j+dj-1) as usize] = true;
                        pushed[(i+di) as usize][(j+dj) as usize] = true;
                    }
                    grid1[i as usize][j as usize] = new_content;
                }
                _ => panic!("invalid cell"),
            }           
            
        }

        if !is_failed {
            grid = grid1;
        }        
    }
    print_grid(&grid);

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '[' {
                sum += i*100 + j;
            }
        }
    }
    sum as i64
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/15_example.txt"));
        assert_eq!(part1(&input), 10092);
        assert_eq!(part2(&input), 9021);
    }
}
