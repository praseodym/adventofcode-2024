use adventofcode_2024::print_answers;
use itertools::Itertools;
use std::collections::HashMap;

/// https://adventofcode.com/2024/day/21
fn main() {
    print_answers(run(include_str!("../../input/day21/input")));
}

fn run(input: &'static str) -> (u64, u64) {
    let numeric_keypad = [
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let directional_keypad = [vec![' ', '^', 'A'], vec!['<', 'v', '>']];

    let num_paths = build_keypad_paths(&numeric_keypad);
    let dir_paths = build_keypad_paths(&directional_keypad);

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut memo1 = HashMap::new();
    let mut memo2 = HashMap::new();
    for line in input.lines() {
        let a = shortest_sequence(line, 0, 3, &num_paths, &dir_paths, &mut memo1);
        let b = shortest_sequence(line, 0, 26, &num_paths, &dir_paths, &mut memo2);
        let numbers = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        part1_answer += a as u64 * numbers;
        part2_answer += b as u64 * numbers;
    }

    (part1_answer, part2_answer)
}

fn shortest_sequence(
    sequence: &str,
    depth: usize,
    max_depth: usize,
    num: &HashMap<(char, char), Vec<String>>,
    dir: &HashMap<(char, char), Vec<String>>,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    let memo_key = (sequence.to_string(), depth);
    if depth == max_depth {
        return sequence.len();
    } else if let Some(&len) = memo.get(&memo_key) {
        return len;
    }

    let keypad = if depth == 0 { num } else { dir };
    let s = 'A'.to_string() + sequence;
    let mut res = 0;
    for (a, b) in s.chars().tuple_windows() {
        let paths = keypad.get(&(a, b)).unwrap();
        let min = paths
            .iter()
            .map(|x| shortest_sequence(x, depth + 1, max_depth, num, dir, memo))
            .min()
            .unwrap();
        res += min;
    }
    memo.insert(memo_key, res);
    res
}

fn build_keypad_paths(keypad: &[Vec<char>]) -> HashMap<(char, char), Vec<String>> {
    let mut key_presses = HashMap::new();
    for (y1, row) in keypad.iter().enumerate() {
        for (x1, &c1) in row.iter().enumerate() {
            for (y2, row) in keypad.iter().enumerate() {
                for (x2, &c2) in row.iter().enumerate() {
                    if c1 == ' ' || c2 == ' ' {
                        continue;
                    }

                    let dx = x2 as i32 - x1 as i32;
                    let dy = y2 as i32 - y1 as i32;

                    let mut keys = vec![];
                    let mut stack = vec![((dx, dy), String::new())];

                    while let Some(((dx, dy), k)) = stack.pop() {
                        if dx == 0 && dy == 0 {
                            keys.push(k + "A");
                            continue;
                        }
                        if keypad[(y2 as i32 - dy) as usize][(x2 as i32 - dx) as usize] == ' ' {
                            continue;
                        }

                        if dx < 0 {
                            stack.push(((dx + 1, dy), k.clone() + "<"));
                        }
                        if dx > 0 {
                            stack.push(((dx - 1, dy), k.clone() + ">"));
                        }

                        if dy < 0 {
                            stack.push(((dx, dy + 1), k.clone() + "^"));
                        }
                        if dy > 0 {
                            stack.push(((dx, dy - 1), k.clone() + "v"));
                        }
                    }

                    key_presses.insert((c1, c2), keys);
                }
            }
        }
    }
    key_presses
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day21/input"));
        assert_eq!(part1_answer, 242484);
        assert_eq!(part2_answer, 294209504640384);
    }

    #[test]
    fn example() {
        let (part1_answer, _) = run(include_str!("../../input/day21/example"));
        assert_eq!(part1_answer, 126384);
    }
}
