/// https://adventofcode.com/2024/day/7
fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../../input/day07/input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u64, u64) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for line in input.lines() {
        let (a, b) = line.split_once(":").unwrap();
        let a = a.parse::<u64>().unwrap();
        let b = b
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        part1_answer += solve(a, &b, false);
        part2_answer += solve(a, &b, true);
    }

    (part1_answer, part2_answer)
}

fn solve(a: u64, b: &[u64], concat: bool) -> u64 {
    let mut res = vec![b[0]];
    for n in b.iter().skip(1) {
        let mut new = vec![];
        for m in res.iter().filter(|&&m| m <= a) {
            new.push(m + n);
            new.push(m * n);
            if concat {
                new.push(m * 10u64.pow(n.ilog10() + 1) + n);
            }
        }
        res = new;
    }
    if res.contains(&a) {
        a
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day07/input"));
        assert_eq!(part1_answer, 3351424677624);
        assert_eq!(part2_answer, 204976636995111);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day07/example"));
        assert_eq!(part1_answer, 3749);
        assert_eq!(part2_answer, 11387);
    }
}
