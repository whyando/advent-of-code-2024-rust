use std::collections::HashMap;

fn main() {
    let input = parse(include_str!("../../input/21.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 107934);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 130470079151124);
}

type Input = Vec<String>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

lazy_static::lazy_static! {
    static ref DIRECTIONAL_KEYPAD: Vec<Vec<char>> = vec![
        vec![' ', '^', 'A'],
        vec!['<', 'v', '>'],
    ];

    static ref NUMERIC_KEYPAD: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
}

fn keypad_loc(keypad: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for i in 0..keypad.len() {
        for j in 0..keypad[i].len() {
            if keypad[i][j] == c {
                return (i, j);
            }
        }
    }
    panic!("Invalid char");
}

struct Scope {
    num_keypads: usize,
    f_cache: HashMap<(usize, char, char), i64>,
}

impl Scope {
    fn new(num_keypads: usize) -> Self {
        Self {
            num_keypads,
            f_cache: HashMap::new(),
        }
    }

    // Number of (human) presses to move keypad r from char x to char y (and enter y on that keypad)
    // (robots < r start and end at 'A', and robots > r are unchanged)
    fn f(&mut self, r: usize, x: char, y: char) -> i64 {
        if r == 0 {
            return 1;
        }
        if let Some(&res) = self.f_cache.get(&(r, x, y)) {
            return res;
        }
        // println!("f({}, {}, {})", r, x, y);
        // robot n-1 is at a numeric keypad
        // rest of the robots are at directional keypads

        let keypad: &Vec<Vec<char>> = if r == self.num_keypads - 1 {
            &NUMERIC_KEYPAD
        } else {
            &DIRECTIONAL_KEYPAD
        };

        // consider the 2 candidate routes where we go one direction and then the other, and don't go off the edge
        let (i_start, j_start) = keypad_loc(keypad, x);
        let (i_end, j_end) = keypad_loc(keypad, y);

        // i first
        let mut route1 = vec!['A'];
        let mut route1_failed = false;
        {
            let mut i = i_start;
            let mut j = j_start;
            while i != i_end {
                if i < i_end {
                    route1.push('v');
                    i += 1;
                } else if i > i_end {
                    route1.push('^');
                    i -= 1;
                }
                if keypad[i as usize][j as usize] == ' ' {
                    route1_failed = true;
                }
            }
            while j != j_end {
                if j < j_end {
                    route1.push('>');
                    j += 1;
                } else if j > j_end {
                    route1.push('<');
                    j -= 1;
                }
                if keypad[i as usize][j as usize] == ' ' {
                    route1_failed = true;
                }
            }
            route1.push('A');
        }

        // j first
        let mut route2 = vec!['A'];
        let mut route2_failed = false;
        {
            let mut i = i_start;
            let mut j = j_start;
            while j != j_end {
                if j < j_end {
                    route2.push('>');
                    j += 1;
                } else if j > j_end {
                    route2.push('<');
                    j -= 1;
                }
                if keypad[i as usize][j as usize] == ' ' {
                    route2_failed = true;
                }
            }
            while i != i_end {
                if i < i_end {
                    route2.push('v');
                    i += 1;
                } else if i > i_end {
                    route2.push('^');
                    i -= 1;
                }
                if keypad[i as usize][j as usize] == ' ' {
                    route2_failed = true;
                }
            }
            route2.push('A');
        }

        let mut min_presses = i64::MAX;
        if !route1_failed {
            let mut presses = 0;
            for i in 0..route1.len() - 1 {
                presses += self.f(r - 1, route1[i], route1[i + 1]);
            }
            min_presses = min_presses.min(presses);
        }
        if !route2_failed {
            let mut presses = 0;
            for i in 0..route2.len() - 1 {
                presses += self.f(r - 1, route2[i], route2[i + 1]);
            }
            min_presses = min_presses.min(presses);
        }
        // println!("routes: {:?}, {:?}, {} {}", route1, route2, route1_failed, route2_failed);
        assert!(min_presses != i64::MAX); // 1 of the 2 routes must be valid

        self.f_cache.insert((r, x, y), min_presses);
        min_presses
    }
}

fn solve(code: &str, num_keypads: usize) -> i64 {
    let mut presses = 0;
    let mut pos = 'A';
    let mut x = Scope::new(num_keypads);
    for i in 0..code.len() {
        let next: char = code.chars().nth(i).unwrap();
        presses += x.f(num_keypads - 1, pos, next);
        pos = next;
    }
    presses
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for code in input {
        let len = code.len();
        let numeric_part: i64 = code[..len - 1].to_string().parse().unwrap();
        let dist = solve(&code, 4);
        println!("{}: {}", code, dist);
        sum += dist * numeric_part;
    }
    sum
}

fn part2(input: &Input) -> i64 {
    let mut sum = 0;
    for code in input {
        let len = code.len();
        let numeric_part: i64 = code[..len - 1].to_string().parse().unwrap();
        let dist = solve(&code, 27);
        println!("{}: {}", code, dist);
        sum += dist * numeric_part;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/21_example.txt"));
        assert_eq!(part1(&input), 126384);
        assert_eq!(part2(&input), 0);
    }
}
