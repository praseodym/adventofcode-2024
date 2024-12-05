/// https://adventofcode.com/2024/day/5
fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../../input/day05/input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let s = input.split_once("\n\n").unwrap();
    let rules = s.0;
    let updates = s.1;

    let rules = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    for update in updates.lines() {
        let u = update
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut valid = true;
        'update: for i in 1..u.len() {
            for rule in rules.iter().filter(|&r| r.0 == u[i]) {
                if u[0..i].contains(&rule.1) {
                    valid = false;
                    break 'update;
                }
            }
        }

        if valid {
            part1_answer += u[u.len() / 2];
        } else {
            let mut u = u.clone();
            'fix: loop {
                for i in 1..u.len() {
                    for rule in rules.iter().filter(|&r| r.0 == u[i]) {
                        if let Some(j) = u[0..i].iter().position(|&x| x == rule.1) {
                            u.swap(i, j);
                            continue 'fix;
                        }
                    }
                }
                break;
            }
            part2_answer += u[u.len() / 2];
        }
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day05/input"));
        assert_eq!(part1_answer, 5391);
        assert_eq!(part2_answer, 6142);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day05/example"));
        assert_eq!(part1_answer, 143);
        assert_eq!(part2_answer, 123);
    }
}
