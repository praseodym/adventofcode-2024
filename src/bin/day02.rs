use adventofcode_2024::print_answers;

/// https://adventofcode.com/2024/day/2
fn main() {
    print_answers(run(include_str!("../../input/day02/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for line in input.lines() {
        let report = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if is_safe(&report) {
            part1_answer += 1;
            part2_answer += 1;
        } else {
            for j in 0..report.len() {
                let mut report = report.clone();
                report.remove(j);
                if is_safe(&report) {
                    part2_answer += 1;
                    break;
                }
            }
        }
    }

    (part1_answer, part2_answer)
}

fn is_safe(report: &[u32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for i in 1..report.len() {
        let a = report[i].abs_diff(report[i - 1]);
        if !(1..=3).contains(&a) {
            return false;
        }
        if report[i] < report[i - 1] {
            increasing = false;
        }
        if report[i] > report[i - 1] {
            decreasing = false;
        }
        if !increasing && !decreasing {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day02/input"));
        assert_eq!(part1_answer, 442);
        assert_eq!(part2_answer, 493);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day02/example"));
        assert_eq!(part1_answer, 2);
        assert_eq!(part2_answer, 4);
    }
}
