fn main() {
    let input = parse(include_str!("../../input/25.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 2770);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 0);
}


type Input = Vec<Vec<Vec<char>>>;

fn parse(input: &str) -> Input {
    let i: Input = input.split("\n\n").map(|group| {
        group.lines().filter(
            |line| !line.is_empty()
        ).
            map(|line| {
            line.chars().collect()
        }).collect()
    }).collect();
    for schematic in &i {
        assert_eq!(schematic.len(), 7);
        for line in schematic {
            assert_eq!(line.len(), 5);
        }
    }
    i
}

fn part1(input: &Input) -> i64 {
    // sort out keys and locks, and convert to heights
    let mut keys = vec![];
    let mut locks = vec![];
    for schematic in input {
        if schematic[0][0] == '#' {
            let mut lock = vec![-1; 5];
            for j in 0..5 {
                let mut i = 0;
                while schematic[i + 1][j] == '#' {
                    i += 1;
                }
                lock[j] = i as i64;
            }
            locks.push(lock);
        } else {
            let mut key = vec![-1; 5];
            for j in 0..5 {
                let mut i = 0;
                while schematic[5 - i][j] == '#' {
                    i += 1;
                }
                key[j] = i as i64;
            }
            keys.push(key);
        }
    }
    // println!("{:?}", locks);
    // println!("{:?}", keys);
    
    let mut count = 0;
    for key in &keys {
        for lock in &locks {
            let mut fit = true;
            for j in 0..5 {
                if key[j] + lock[j] > 5 {
                    fit = false;
                    break;
                }
            }
            if fit {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/25_example.txt"));
        assert_eq!(part1(&input), 3);
        assert_eq!(part2(&input), 0);
    }
}
