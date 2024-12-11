use adventofcode_2024::print_answers;
use std::collections::HashMap;

/// https://adventofcode.com/2024/day/11
fn main() {
    print_answers(run(include_str!("../../input/day11/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut part1_answer = 0;
    let mut part2_answer = 0;
    let mut memo = HashMap::new();
    for stone in stones.iter() {
        part1_answer += split_stone(*stone, 25, &mut memo);
        part2_answer += split_stone(*stone, 75, &mut memo);
    }

    (part1_answer, part2_answer)
}

fn split_stone(stone: u64, iterations: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(&result) = memo.get(&(stone, iterations)) {
        return result;
    }

    let result = if iterations == 0 {
        1
    } else if stone == 0 {
        split_stone(1, iterations - 1, memo)
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let f = 10u64.pow((stone.ilog10() + 1) / 2);
        let (left, right) = (stone / f, stone % f);
        split_stone(left, iterations - 1, memo) + split_stone(right, iterations - 1, memo)
    } else {
        split_stone(stone * 2024, iterations - 1, memo)
    };

    memo.insert((stone, iterations), result);
    result
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day11/input"));
        assert_eq!(part1_answer, 186424);
        assert_eq!(part2_answer, 219838428124832);
    }

    #[test]
    fn example() {
        let (part1_answer, _) = run(include_str!("../../input/day11/example"));
        assert_eq!(part1_answer, 55312);
    }
}
