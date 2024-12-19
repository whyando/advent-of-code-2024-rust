/*

https://adventofcode.com/2024/day/14

part 1:
Easy computation

part 2:
Very interesting and unique problem. It's tricky to understand how to computationally tell when the picture emerges. But the
hint is from part 1 where we are computing a basic 'entropy' of the picture. We then just have to find t where the entropy is minimized.

the x coords are periodic every 101 values and the y coords are periodic every 103 values. So the max period is 101*103 = 10303

*/

fn main() {
    let input = parse(include_str!("../../input/14.txt"));

    let part1 = part1(&input, 101, 103);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 232589280);

    let part2 = part2(&input, 101, 103);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 7569);
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

type Input = Vec<Robot>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            // p=0,4 v=3,-3
            let re = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
            let caps = re.captures(line).unwrap();
            Robot {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                vx: caps[3].parse().unwrap(),
                vy: caps[4].parse().unwrap(),
            }
        })
        .collect()
}

fn part1(input: &Input, width: i64, height: i64) -> i64 {
    let mut grid = vec![vec![0; width as usize]; height as usize];
    let t = 100;
    // for each robot, we can calculate the position at time t
    for robot in input {
        let x = (robot.x + robot.vx * t).rem_euclid(width);
        let y = (robot.y + robot.vy * t).rem_euclid(height);
        grid[y as usize][x as usize] += 1;
    }

    // print grid
    for row in &grid {
        for cell in row {
            if *cell > 0 {
                print!("{}", cell);
            } else {
                print!(".");
            }
        }
        println!();
    }

    // count robots in each quadrant
    let mut quadrants = vec![0; 4];
    for y in 0..height {
        for x in 0..width {
            let y_cmp = (2 * y + 1).cmp(&height);
            let x_cmp = (2 * x + 1).cmp(&width);
            let quadrant = match (x_cmp, y_cmp) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 2,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3,
                _ => {
                    // skip center
                    continue;
                }
            };
            quadrants[quadrant] += grid[y as usize][x as usize];
        }
    }

    let mut product = 1;
    for quadrant in quadrants {
        product *= quadrant;
    }
    product
}

fn part2(input: &Input, width: i64, height: i64) -> i64 {
    let mut least_entropy = std::i64::MAX;
    let mut least_entropy_t = 0;
    for t in 0..=height * width {
        let mut grid = vec![vec![0; width as usize]; height as usize];
        // for each robot, we can calculate the position at time t
        for robot in input {
            let x = (robot.x + robot.vx * t).rem_euclid(width);
            let y = (robot.y + robot.vy * t).rem_euclid(height);
            grid[y as usize][x as usize] += 1;
        }

        // count robots in each quadrant
        let mut quadrants = vec![0; 4];
        for y in 0..height {
            for x in 0..width {
                let y_cmp = (2 * y + 1).cmp(&height);
                let x_cmp = (2 * x + 1).cmp(&width);
                let quadrant = match (x_cmp, y_cmp) {
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 1,
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 2,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3,
                    _ => {
                        // skip center
                        continue;
                    }
                };
                quadrants[quadrant] += grid[y as usize][x as usize];
            }
        }
        let mut entropy = 1;
        for quadrant in quadrants {
            entropy *= quadrant;
        }
        if entropy < least_entropy {
            println!("t = {}, entropy = {}", t, entropy);
            least_entropy = entropy;
            least_entropy_t = t;

            // print grid
            for row in &grid {
                for cell in row {
                    if *cell > 0 {
                        print!("{}", cell);
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }

    least_entropy_t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/14_example.txt"));
        assert_eq!(part1(&input, 11, 7), 12);
        assert_eq!(part2(&input, 11, 7), 0); // No picture emerges in the test example
    }
}
