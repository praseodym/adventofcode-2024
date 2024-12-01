fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../../input/day01/input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut parsed = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        left.push(parsed.next().unwrap());
        right.push(parsed.next().unwrap());
    }

    left.sort();
    right.sort();

    let mut part1_answer = 0;
    for i in 0..left.len() {
        part1_answer += left[i].abs_diff(right[i]);
    }

    let mut part2_answer = 0;
    for &n in left.iter() {
        part2_answer += n * right.iter().filter(|&m| *m == n).count() as u32;
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day01/input"));
        assert_eq!(part1_answer, 1879048);
        assert_eq!(part2_answer, 21024792);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day01/example"));
        assert_eq!(part1_answer, 11);
        assert_eq!(part2_answer, 31);
    }
}
