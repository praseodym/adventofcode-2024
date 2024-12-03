/// https://adventofcode.com/2024/day/3
fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../../input/day03/input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut disabled = vec![];
    for (i, _) in input.match_indices("don't()") {
        let do_index = input[i..].find("do()").unwrap();
        disabled.push(i..i + do_index);
    }

    let re = regex::Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").unwrap();
    for c in re.captures_iter(input) {
        let x = c["x"].parse::<u32>().unwrap();
        let y = c["y"].parse::<u32>().unwrap();
        part1_answer += x * y;

        let idx = c.get(0).unwrap().start();
        if !disabled.iter().any(|range| range.contains(&idx)) {
            part2_answer += x * y;
        }
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day03/input"));
        assert_eq!(part1_answer, 166357705);
        assert_eq!(part2_answer, 88811886);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _) = run(include_str!("../../input/day03/example1"));
        assert_eq!(part1_answer, 161);
        let (_, part2_answer) = run(include_str!("../../input/day03/example2"));
        assert_eq!(part2_answer, 48);
    }
}
