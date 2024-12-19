use adventofcode_2024::print_answers;
use std::collections::HashMap;

/// https://adventofcode.com/2024/day/19
fn main() {
    print_answers(run(include_str!("../../input/day19/input")));
}

fn run(input: &'static str) -> (u32, u64) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let towels = a.split(", ").collect::<Vec<_>>();
    let designs = b.lines().collect::<Vec<_>>();

    let mut part1_answer = 0;
    let mut part2_answer = 0;
    let mut memo = HashMap::new();
    for design in designs {
        let n = possibilities(&towels, design, &mut memo);
        if n > 0 {
            part1_answer += 1;
            part2_answer += n;
        }
    }

    (part1_answer, part2_answer)
}

fn possibilities(
    towels: &[&str],
    design: &'static str,
    memo: &mut HashMap<&'static str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    } else if let Some(&result) = memo.get(design) {
        return result;
    }

    let possibilities = towels
        .iter()
        .filter_map(|towel| design.strip_prefix(towel))
        .map(|rest| possibilities(towels, rest, memo))
        .sum();
    memo.insert(design, possibilities);
    possibilities
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day19/input"));
        assert_eq!(part1_answer, 306);
        assert_eq!(part2_answer, 604622004681855);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day19/example"));
        assert_eq!(part1_answer, 6);
        assert_eq!(part2_answer, 16);
    }
}
