/*

https://adventofcode.com/2024/day/20

part 1:
pathfinding, distance with each wall removed. I guess the trivial solution is to run pathfinding N^2 times which is fast enough for
part 1. I went straight for a double dijkstra solution.

part 2:
double dijsktra solution generalizes easily to part 2

*/

fn main() {
    let input = parse(include_str!("../../input/20.txt"));

    let part1 = part1(&input, 100);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1395);

    let part2 = part2(&input, 20, 100);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 993178);
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn get_pos(input: &Input, c: char) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&x| x == c).map(|x| (x, y)))
        .unwrap()
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dijkstra(input: &Input, start: (usize, usize)) -> Vec<Vec<i64>> {    
    let mut dist = vec![vec![std::i64::MAX; input[0].len()]; input.len()];
    let mut locked = vec![vec![false; input[0].len()]; input.len()];
    let mut queue = std::collections::BinaryHeap::new();
    queue.push((0, start));
    while queue.len() > 0 {
        let (d, (x, y)) = queue.pop().unwrap();
        if locked[y][x] {
            continue;
        }
        locked[y][x] = true;
        dist[y][x] = d;
        for (dx, dy) in &DIRECTIONS {
            let x1 = (x as i64 + dx) as usize;
            let y1 = (y as i64 + dy) as usize;
            if x1 < input[0].len() && y1 < input.len() {
                if input[y1][x1] != '#' {
                    queue.push((d + 1, (x1, y1)));
                }
            }
        }
    }
    dist
}

fn part1(input: &Input, threshold: i64) -> i64 {
    part2(input, 2, threshold)
}

fn part2(input: &Input, max_cheat_dist: i64, threshold: i64) -> i64 {
    let start = get_pos(input, 'S');
    let end = get_pos(input, 'E');

    let dist = dijkstra(input, start);
    let dist_end = dijkstra(input, end);

    let time = dist[end.1][end.0];
    assert_eq!(time, dist_end[start.1][start.0]);

    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if dist[y][x] == std::i64::MAX {
                continue;
            }
            // (y, x) is 'cheat start'
            // and (y1, x1) is 'cheat end'
            for cheat_x in -max_cheat_dist..=max_cheat_dist {
                for cheat_y in -max_cheat_dist..=max_cheat_dist {
                    if cheat_x.abs() + cheat_y.abs() > max_cheat_dist {
                        continue;
                    }
                    let x1 = (x as i64 + cheat_x) as usize;
                    let y1 = (y as i64 + cheat_y) as usize;
                    if x1 < input[0].len() && y1 < input.len() {
                        if dist[y1][x1] != std::i64::MAX && dist_end[y1][x1] != std::i64::MAX {
                            let route = dist[y][x] + dist_end[y1][x1] + cheat_x.abs() + cheat_y.abs();
                            let saving = time - route;
                            if saving >= threshold {
                                count += 1;
                            }
                        }
                    }
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
        let input = parse(include_str!("../../input/20_example.txt"));
        assert_eq!(part1(&input, 20), 5);
        assert_eq!(part2(&input, 20, 70), 41);
    }
}
