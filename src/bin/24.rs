use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = parse(include_str!("../../input/24.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 51107420031718);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 0);
}


#[derive(Debug)]
struct Input {
    wires: BTreeMap<String, bool>,
    gates: BTreeMap<String, (String, String, String)>,
}

fn parse(input: &str) -> Input {
    // split on double newline
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);

    let wires = parts[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            assert_eq!(parts.len(), 2);
            let val: i64 = parts[1].parse::<i64>().unwrap();
            assert!(val == 0 || val == 1);
            (parts[0].to_string(), val == 1)
        })
        .collect();
    let gates = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let regex = regex::Regex::new(r"^(.+) (AND|XOR|OR) (.+) -> (.+)$").unwrap();
            let captures = regex.captures(line).unwrap();
            let input1 = captures.get(1).unwrap().as_str();
            let operation = captures.get(2).unwrap().as_str();
            let input2 = captures.get(3).unwrap().as_str();
            let output = captures.get(4).unwrap().as_str();
            (output.to_string(), (input1.to_string(), operation.to_string(), input2.to_string()))
        })
        .collect();
    Input {
        wires,
        gates,
    }
}

fn f(wires: &mut BTreeMap<String, bool>, gates: &BTreeMap<String, (String, String, String)>, x: &str) -> bool {
    if let Some(val) = wires.get(x) {
        return *val;
    }

    let (input1, operation, input2) = gates.get(x).unwrap();
    let input1_val = f(wires, gates, input1);
    let input2_val = f(wires, gates, input2);

    let val = match operation.as_str() {
        "AND" => input1_val & input2_val,
        "OR" => input1_val | input2_val,
        "XOR" => input1_val ^ input2_val,
        _ => panic!("Unknown operation: {}", operation),
    };
    wires.insert(x.to_string(), val);
    val
}

fn part1(input: &Input) -> i64 {
    let mut wires = input.wires.clone();
    let mut sum = 0;
    for output in input.gates.keys() {
        let val = f(&mut wires, &input.gates, output);
        if output.chars().nth(0).unwrap() == 'z' {
            // println!("{}: {}", output, val);
            let index = output.chars().skip(1).collect::<String>().parse::<i64>().unwrap();
            if val {
                sum += 1 << index;
            }
        }
    }
    sum
}

fn f1(
    reached: &mut BTreeSet<String>, 
    order: &mut BTreeMap<String, usize>,
    gates: &BTreeMap<String, (String, String, String)>,
    x: &str,
    n: usize,
) {
    if reached.contains(x) {
        return;
    }
    reached.insert(x.to_string());
    if let Some((input1, _op, input2)) = gates.get(x) {
        f1(reached, order, gates, input1, n);
        f1(reached, order, gates, input2, n);
    }
    assert!(!order.contains_key(x));
    order.insert(x.to_string(), order.len());
}

fn swap(gates: &mut BTreeMap<String, (String, String, String)>, x: &str, y: &str) {
    let gate1 = gates.get(x).unwrap().clone();
    let gate2 = gates.get(y).unwrap().clone();
    gates.insert(x.to_string(), gate2);
    gates.insert(y.to_string(), gate1);
}

fn part2(input: &Input) -> i64 {
    // apply gate swaps
    let gates = {
        let mut gates = input.gates.clone();
        swap(&mut gates, "z10", "gpr");
        swap(&mut gates, "z33", "ghp");
        swap(&mut gates, "z21", "nks");
        swap(&mut gates, "krs", "cpm");
        gates
    };

    // sort all wires in topological order
    let mut reached = BTreeSet::new();
    let mut order = BTreeMap::new();
    let n = input.gates.len() + input.wires.len();
    // for output in input.gates.keys() {
    //     f1(&mut reached, &mut order, &input.gates, output, n);
    // }
    let mut seen = BTreeSet::new();
    for i in 0..=45 {
        println!("z{:02}", i);
        f1(&mut reached, &mut order, &gates, &format!("z{:02}", i), n);
        
        let mut new_gates = vec![];
        for i in 0..order.len() {
            let x = order.iter().find(|(_k, v)| **v == i).unwrap().0;
            if seen.contains(x) {
                continue;
            }
            seen.insert(x.to_string());
            if let Some(gate) = gates.get(x) {
                println!("{} = {} {} {}", x, gate.0, gate.1, gate.2);
                new_gates.push((x.to_string(), gate.clone()));
            }
        }
        println!();

        // analyse:
        let num_and = new_gates.iter().filter(|(_k, (_i1, op, _i2))| op == "AND").count();
        let num_or = new_gates.iter().filter(|(_k, (_i1, op, _i2))| op == "OR").count();
        let num_xor = new_gates.iter().filter(|(_k, (_i1, op, _i2))| op == "XOR").count();
        if i >= 2 && i != 45 {
            assert_eq!(num_and, 2);
            assert_eq!(num_or, 1);
            assert_eq!(num_xor, 2);
        }
    }    

    let mut swaps = vec!["z10", "gpr", "z33", "ghp", "z21", "nks", "krs", "cpm"];
    swaps.sort();
    let ans = swaps.join(",");
    println!("{}", ans);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/24_example.txt"));
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test2() {
        let input = parse(include_str!("../../input/24_example2.txt"));
        assert_eq!(part1(&input), 2024);
    }
}
