use adventofcode_2024::print_answers;

/// https://adventofcode.com/2024/day/13
fn main() {
    print_answers(run(include_str!("../../input/day13/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut lines = input.lines().peekable();
    let re_a = regex::Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_b = regex::Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_prize = regex::Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        let c = re_a.captures(line).unwrap();
        let ax = c[1].parse::<i64>().unwrap();
        let ay = c[2].parse::<i64>().unwrap();

        let line = lines.next().unwrap();
        let c = re_b.captures(line).unwrap();
        let bx = c[1].parse::<i64>().unwrap();
        let by = c[2].parse::<i64>().unwrap();

        let line = lines.next().unwrap();
        let c = re_prize.captures(line).unwrap();
        let px = c[1].parse::<i64>().unwrap();
        let py = c[2].parse::<i64>().unwrap();

        lines.next();

        part1_answer += min_tokens(ax, ay, bx, by, px, py);
        let offset = 10000000000000;
        part2_answer += min_tokens(ax, ay, bx, by, px + offset, py + offset);
    }

    (part1_answer, part2_answer)
}

fn min_tokens(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> u64 {
    // Solve this system of two linear equations:
    // ax * a + bx * b = px
    // ay * a + by * b = py
    // https://en.wikipedia.org/wiki/Cramer%27s_rule#Explicit_formulas_for_small_systems
    let det = ax * by - bx * ay;
    let det_a = px * by - bx * py;
    let det_b = ax * py - px * ay;
    if det_a % det == 0 && det_b % det == 0 {
        (det_a / det * 3 + det_b / det) as u64
    } else {
        0
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day13/input"));
        assert_eq!(part1_answer, 36250);
        assert_eq!(part2_answer, 83232379451012);
    }

    #[test]
    fn example() {
        let (part1_answer, _) = run(include_str!("../../input/day13/example"));
        assert_eq!(part1_answer, 480);
    }
}
