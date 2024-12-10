fn main() {
    let input = parse(include_str!("../../input/05.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 6242);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 5169);
}

#[derive(Debug)]
struct Input {
    rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

fn parse(input: &str) -> Input {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        // Rules
        if line.contains("|") {
            let mut parts = line.split('|');
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            rules.push((x.parse().unwrap(), y.parse().unwrap()));
        }
        // Updates
        else if line.contains(",") {
            let parts = line.split(',');
            let mut update = Vec::new();
            for part in parts {
                update.push(part.parse().unwrap());
            }
            assert!(update.len() % 2 == 1);
            updates.push(update);
        }
    }
    Input { rules, updates }
}

fn is_correct_update(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> Option<(usize, usize)> {
    for (x, y) in rules {
        let x_index = update.iter().position(|&n| n == *x);
        let y_index = update.iter().position(|&n| n == *y);
        match (x_index, y_index) {
            (Some(x_index), Some(y_index)) => {
                if x_index > y_index {
                    return Some((x_index, y_index));
                }
            }
            _ => {}
        }
    }
    None
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for update in &input.updates {
        if is_correct_update(update, &input.rules).is_none() {
            let middle = update[update.len() / 2];
            sum += middle;
        }
    }
    sum
}

fn part2(input: &Input) -> i64 {
    let mut sum = 0;
    for update in &input.updates {
        let mut update = update.clone();
        let starts_correct = is_correct_update(&update, &input.rules).is_none();
        if starts_correct {
            continue;
        }
        loop {
            if let Some((x, y)) = is_correct_update(&update, &input.rules) {
                update.swap(x, y);
                // println!("swapped {:?} -> {:?}", x, y);
            } else {
                break;
            }
        }
        let middle = update[update.len() / 2];
        sum += middle;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/05_example.txt"));
        assert_eq!(part1(&input), 143);
        assert_eq!(part2(&input), 123);
    }
}
