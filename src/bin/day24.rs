use adventofcode_2024::print_answers;
use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::VecDeque;

/// https://adventofcode.com/2024/day/24
fn main() {
    print_answers(run(include_str!("../../input/day24/input")));
}

fn run(input: &'static str) -> (u64, String) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let mut wires = a
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(k, v)| (k, v != "0"))
        .collect::<HashMap<_, _>>();
    let mut gates = VecDeque::new();
    let re = regex::Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
    for (_, [a, op, b, c]) in re.captures_iter(b).map(|c| c.extract()) {
        gates.push_back((a, op, b, c));
    }

    let mut remaining = gates.clone();
    while let Some((a, op, b, c)) = remaining.pop_front() {
        if let (Some(a), Some(b)) = (wires.get(a), wires.get(b)) {
            let out = match op {
                "AND" => a & b,
                "OR" => a | b,
                "XOR" => a ^ b,
                _ => unreachable!("unknown op: {}", op),
            };
            wires.insert(c, out);
        } else {
            remaining.push_back((a, op, b, c));
        }
    }

    let part1_answer = wires
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .map(|(_, &v)| v)
        .fold(0, |acc, v| (acc << 1) | v as u64);

    let mut wrong = vec![];
    let max_z = *wires.keys().filter(|k| k.starts_with("z")).max().unwrap();
    for (a, op, b, c) in &gates {
        if *op != "XOR" && c.starts_with("z") && *c != max_z {
            wrong.push(c);
        } else if *op == "AND" && *a != "x00" && *b != "x00" {
            for (a2, op2, b2, _) in &gates {
                if (a2 == c || b2 == c) && *op2 != "OR" {
                    wrong.push(c);
                    break;
                }
            }
        } else if *op == "XOR" {
            let xyz = ['x', 'y', 'z'];
            if !a.starts_with(xyz) && !b.starts_with(xyz) && !c.starts_with(xyz) {
                wrong.push(c);
            } else {
                for (a2, op2, b2, _) in &gates {
                    if (a2 == c || b2 == c) && *op2 == "OR" {
                        wrong.push(c);
                        break;
                    }
                }
            }
        }
    }
    let part2_answer = wrong.iter().sorted().dedup().join(",");

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day24/input"));
        assert_eq!(part1_answer, 58639252480880);
        assert_eq!(part2_answer, "bkr,mqh,rnq,tfb,vvr,z08,z28,z39");
    }

    #[test]
    fn example1() {
        let (part1_answer, _) = run(include_str!("../../input/day24/example1"));
        assert_eq!(part1_answer, 4);
    }

    #[test]
    fn example2() {
        let (part1_answer, _) = run(include_str!("../../input/day24/example2"));
        assert_eq!(part1_answer, 2024);
    }
}
