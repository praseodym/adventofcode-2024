use adventofcode_2024::{grid, print_answers};
use std::collections::{HashSet, VecDeque};

/// https://adventofcode.com/2024/day/10
fn main() {
    print_answers(run(include_str!("../../input/day10/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let (grid, len) = grid::parse_char_grid(input);

    let mut trailheads = Vec::new();
    grid.iter().enumerate().for_each(|(i, &c)| {
        if c == b'0' {
            let x = i % len;
            let y = i / len;
            trailheads.push((x, y));
        }
    });

    let mut part1_answer = 0;
    let mut part2_answer = 0;
    for &(x, y) in &trailheads {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((x, y));
        while let Some((x, y)) = queue.pop_front() {
            let c = grid[y * len + x];
            if c == b'9' {
                if seen.insert((x, y)) {
                    part1_answer += 1;
                }
                part2_answer += 1;
            }
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if new_x < 0 || new_x >= len as isize || new_y < 0 || new_y >= len as isize {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if grid[new_y * len + new_x] == c + 1 {
                    queue.push_back((new_x, new_y));
                }
            }
        }
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day10/input"));
        assert_eq!(part1_answer, 822);
        assert_eq!(part2_answer, 1801);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _) = run(include_str!("../../input/day10/example1"));
        assert_eq!(part1_answer, 1);
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day10/example2"));
        assert_eq!(part1_answer, 36);
        assert_eq!(part2_answer, 81);
    }

    #[test]
    fn test_input_example3() {
        let (_, part2_answer) = run(include_str!("../../input/day10/example3"));
        assert_eq!(part2_answer, 13);
    }

    #[test]
    fn test_input_example4() {
        let (_, part2_answer) = run(include_str!("../../input/day10/example4"));
        assert_eq!(part2_answer, 227);
    }
}
