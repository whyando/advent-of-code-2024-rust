use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = parse(include_str!("../../input/23.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1306);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, "bd,dk,ir,ko,lk,nn,ob,pt,te,tl,uh,wj,yl");
}

type Input = Vec<(String, String)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split("-");
            let r = (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            );
            assert!(parts.next().is_none());
            r
        })
        .collect()
}

fn part1(input: &Input) -> i64 {
    let mut adj = BTreeMap::new();
    let mut edges = BTreeSet::new();
    for (a, b) in input {
        let e = adj.entry(a).or_insert_with(Vec::new);
        e.push(b);
        let e = adj.entry(b).or_insert_with(Vec::new);
        e.push(a);
        edges.insert((a, b));
        edges.insert((b, a));
    }

    let mut results = BTreeSet::new();
    for (&k, v) in adj.iter() {
        if k.chars().nth(0).unwrap() == 't' {
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    if edges.contains(&(v[i], v[j])) {
                        // println!("{} {} {}", k, v[i], v[j]);
                        // sort
                        let mut v = vec![k.clone(), v[i].clone(), v[j].clone()];
                        v.sort();
                        results.insert(v);
                    }
                }
            }
        }
    }
    // results.iter().for_each(|v| println!("{:?}", v));
    results.len() as i64
}

fn part2(input: &Input) -> String {
    // enumerate all computers
    let mut map = BTreeMap::new();
    let mut map_rev = BTreeMap::new();
    let mut count = 0;
    for (a, b) in input {
        if !map.contains_key(a) {
            map.insert(a, count);
            map_rev.insert(count, a);
            count += 1;
        }
        if !map.contains_key(b) {
            map.insert(b, count);
            map_rev.insert(count, b);
            count += 1;
        }
    }

    // Make adjacency matrix
    let mut matrix = vec![vec![false; count]; count];
    for (a, b) in input {
        let a = map[a];
        let b = map[b];
        matrix[a][b] = true;
        matrix[b][a] = true;
    }

    let mut groups = vec![];
    for i in 0..count {
        groups.push(vec![i]);
    }
    for group_sz in 2.. {
        let mut groups1 = vec![];
        for group in &groups {
            let last = group[group_sz - 2];
            for i in last + 1..count {
                if group.iter().all(|&j| matrix[i][j]) {
                    let mut new_group = group.clone();
                    new_group.push(i);
                    groups1.push(new_group);
                }
            }
        }
        groups = groups1;
        println!("{} groups of size {}", groups.len(), group_sz);
        if groups.len() <= 1 {
            break;
        }
    }

    let group = groups.get(0).unwrap();
    let mut group = group
        .iter()
        .map(|&i| map_rev[&i].clone())
        .collect::<Vec<_>>();
    group.sort();
    group.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/23_example.txt"));
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), "co,de,ka,ta");
    }
}
