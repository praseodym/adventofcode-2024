use adventofcode_2024::print_answers;
use hashbrown::HashMap;

/// https://adventofcode.com/2024/day/22
fn main() {
    print_answers(run(include_str!("../../input/day22/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let mut part1_answer = 0;
    let mut seq_prices: HashMap<[i64; 4], u64> = HashMap::new();
    for line in input.lines() {
        let mut secret = line.parse::<u64>().unwrap();
        let mut changes = vec![];
        let mut seq_prices_local: HashMap<[i64; 4], u64> = HashMap::new();
        for _ in 0..2000 {
            let prev_price = secret % 10;
            secret = ((secret * 64) ^ secret) % 16777216;
            secret = ((secret / 32) ^ secret) % 16777216;
            secret = ((secret * 2048) ^ secret) % 16777216;

            let price = secret % 10;
            let change = prev_price as i64 - price as i64;
            changes.push(change);
            if changes.len() >= 4 {
                let seq = changes[changes.len() - 4..].try_into().unwrap();
                seq_prices_local.entry(seq).or_insert(price);
            }
        }
        part1_answer += secret;

        seq_prices_local.iter().for_each(|(k, v)| {
            *seq_prices.entry(*k).or_default() += v;
        });
    }

    let part2_answer = *seq_prices.values().max().unwrap();

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day22/input"));
        assert_eq!(part1_answer, 17965282217);
        assert_eq!(part2_answer, 2152);
    }

    #[test]
    fn example1() {
        let (part1_answer, _) = run(include_str!("../../input/day22/example1"));
        assert_eq!(part1_answer, 37327623);
    }

    #[test]
    fn example2() {
        let (_, part2_answer) = run(include_str!("../../input/day22/example2"));
        assert_eq!(part2_answer, 23);
    }
}
