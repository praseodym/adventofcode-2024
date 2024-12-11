use adventofcode_2024::print_answers;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// https://adventofcode.com/2024/day/8
fn main() {
    print_answers(run(include_str!("../../input/day08/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let (antennas, len) = parse_input(input);
    let mut antinodes_part1: HashSet<(usize, usize)> = HashSet::new();
    let mut antinodes_part2: HashSet<(usize, usize)> = HashSet::new();
    for antennas in antennas.values() {
        for pair in antennas.iter().permutations(2) {
            let (&(x1, y1), &(x2, y2)) = (pair[0], pair[1]);
            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;
            for i in 0..len as isize {
                let x3 = x2 as isize + dx * i;
                let y3 = y2 as isize + dy * i;
                if x3 < 0 || x3 >= len as isize || y3 < 0 || y3 >= len as isize {
                    break;
                }
                if i == 1 {
                    antinodes_part1.insert((x3 as usize, y3 as usize));
                }
                antinodes_part2.insert((x3 as usize, y3 as usize));
            }
        }
    }
    (antinodes_part1.len() as u32, antinodes_part2.len() as u32)
}

type Antennas = HashMap<u8, Vec<(usize, usize)>>;
fn parse_input(input: &str) -> (Antennas, usize) {
    let len = input.lines().next().unwrap().len();
    assert_eq!(len, input.lines().count(), "input must be square");
    let mut antennas: Antennas = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, c)| {
            if c != b'.' {
                antennas.entry(c).or_default().push((x, y));
            }
        });
    });
    (antennas, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day08/input"));
        assert_eq!(part1_answer, 273);
        assert_eq!(part2_answer, 1017);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day08/example1"));
        assert_eq!(part1_answer, 14);
        assert_eq!(part2_answer, 34);
    }

    #[test]
    fn test_input_example2() {
        let (_, part2_answer) = run(include_str!("../../input/day08/example2"));
        assert_eq!(part2_answer, 9);
    }
}
