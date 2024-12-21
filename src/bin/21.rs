use std::collections::{HashMap, VecDeque};

fn main() {
    let input = parse(include_str!("../../input/21.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 107934);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 0);
}

type Input = Vec<String>;

fn parse(input: &str) -> Input {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string()).collect()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    dir_keypad_1: char,
    dir_keypad_2: char,
    numeric_keypad: char,
    correct_letters: i64,
}
const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>'],
];

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

fn numeric_keypad(current: char, action: char) -> Option<char> {
    let (mut i, mut j) = (-1, -1);
    for i1 in 0..4 {
        for j1 in 0..3 {
            if NUMERIC_KEYPAD[i1][j1] == current {
                i = i1 as i64;
                j = j1 as i64;
            }
        }
    }
    assert!(i != -1 && j != -1);
    let (di, dj) = match action {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Invalid action"),
    };
    let (i1, j1) = (i + di, j + dj);
    if i1 < 0 || i1 >= 4 || j1 < 0 || j1 >= 3 {
        return None;
    }
    let next = NUMERIC_KEYPAD[i1 as usize][j1 as usize];
    if next == ' ' {
        return None;
    }
    Some(next)
}


fn directional_keypad(current: char, action: char) -> Option<char> {
    let (mut i, mut j) = (-1, -1);
    for i1 in 0..2 {
        for j1 in 0..3 {
            if DIRECTIONAL_KEYPAD[i1][j1] == current {
                i = i1 as i64;
                j = j1 as i64;
            }
        }
    }
    assert!(i != -1 && j != -1);
    let (di, dj) = match action {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Invalid action"),
    };
    let (i1, j1) = (i + di, j + dj);
    if i1 < 0 || i1 >= 2 || j1 < 0 || j1 >= 3 {
        return None;
    }
    let next = DIRECTIONAL_KEYPAD[i1 as usize][j1 as usize];
    if next == ' ' {
        return None;
    }
    Some(next)
}

// None if go off the edge, or enter wrong number in the end code
fn apply_action(s: &State, action_0: char, code: &str) -> Option<State> {

    // Press action 0 on keypad 0 which affects keypad 1
    if action_0 != 'A' {
        return match directional_keypad(s.dir_keypad_1, action_0) {
            Some(next) => {
                let mut t = s.clone();
                t.dir_keypad_1 = next;
                Some(t)
            }
            None => None,
        }        
    }

    // Press action 1 on keypad 1 which affects keypad 2
    let action_1 = s.dir_keypad_1;
    if action_1 != 'A' {
        return match directional_keypad(s.dir_keypad_2, action_1) {
            Some(next) => {
                let mut t = s.clone();
                t.dir_keypad_2 = next;
                Some(t)
            }
            None => None,
        }
    }

    // Press action 2 on keypad 2 which affects numeric keypad
    let action_2 = s.dir_keypad_2;
    if action_2 != 'A' {
        return match numeric_keypad(s.numeric_keypad, action_2) {
            Some(next) => {
                let mut t = s.clone();
                t.numeric_keypad = next;
                Some(t)
            }
            None => None,
        }
    }

    // Then the final keypad has been pressed
    let action_3 = s.numeric_keypad;
    let target_letter = code.chars().nth(s.correct_letters as usize).unwrap();
    if action_3 == target_letter {
        let mut t = s.clone();
        t.correct_letters += 1;
        Some(t)
    } else {
        None
    }    
}

fn solve(code: &str) -> i64 {
    let start = State {
        dir_keypad_1: 'A',
        dir_keypad_2: 'A',
        numeric_keypad: 'A',
        correct_letters: 0,
    };

    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();
    queue.push_back((start, 0));

    while let Some((x, d)) = queue.pop_front() {
        if let Some(&prev_dist) = dist.get(&x) {
            assert!(d >= prev_dist);
            continue;
        }
        // println!("{} {} {} {} {}", d, x.dir_keypad_1, x.dir_keypad_2, x.numeric_keypad, x.correct_letters);
        dist.insert(x.clone(), d);

        // Check if terminal state
        if x.correct_letters == code.len() as i64 {
            return d;
        }

        // Consider edge
        for action in vec!['<', '>', 'v', '^', 'A'] {
            if let Some(y) = apply_action(&x, action, &code) {
                queue.push_back((y, d + 1));
            }
        }
    }
    panic!("No solution found");
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for code in input {
        let len = code.len();
        let numeric_part: i64 = code[..len - 1].to_string().parse().unwrap();
        let dist = solve(&code);
        println!("{}: {}", code, dist);
        sum += dist * numeric_part;
    }
    sum
}

fn part2(input: &Input) -> i64 {
    0
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
