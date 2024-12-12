use adventofcode_2024::grid::parse_char_grid;
use adventofcode_2024::print_answers;
use std::collections::{HashSet, VecDeque};

/// https://adventofcode.com/2024/day/12
fn main() {
    print_answers(run(include_str!("../../input/day12/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let (grid, len) = parse_char_grid(input);

    let mut visited = HashSet::new();
    for y in 0..len {
        for x in 0..len {
            if visited.contains(&(x, y)) {
                continue;
            }

            let c = grid[y * len + x];
            let mut area = 0;
            let mut perimeter = 0;
            let mut perimeter_points = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((x as isize, y as isize));
            while let Some((x, y)) = queue.pop_front() {
                if !visited.insert((x as usize, y as usize)) {
                    continue;
                }
                area += 1;

                for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0
                        && ny >= 0
                        && nx < len as isize
                        && ny < len as isize
                        && grid[ny as usize * len + nx as usize] == c
                    {
                        queue.push_back((nx, ny));
                    } else {
                        perimeter += 1;
                        perimeter_points.insert(((x, y), (dx, dy)));
                    }
                }
            }

            let mut sides = 0;
            let mut visited = HashSet::new();
            for ((x, y), d) in perimeter_points.iter().copied() {
                if visited.contains(&((x, y), d)) {
                    continue;
                }
                sides += 1;

                let mut queue = VecDeque::new();
                queue.push_back((x, y));
                while let Some((x, y)) = queue.pop_front() {
                    if !visited.insert(((x, y), d)) {
                        continue;
                    }
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                        let (nx, ny) = (x + dx, y + dy);
                        if perimeter_points.contains(&((nx, ny), d)) {
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }

            part1_answer += area * perimeter;
            part2_answer += area * sides;
        }
    }
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day12/input"));
        assert_eq!(part1_answer, 1457298);
        assert_eq!(part2_answer, 921636);
    }

    #[test]
    fn example1() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day12/example1"));
        assert_eq!(part1_answer, 140);
        assert_eq!(part2_answer, 80);
    }

    #[test]
    fn example2() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day12/example2"));
        assert_eq!(part1_answer, 772);
        assert_eq!(part2_answer, 436);
    }

    #[test]
    fn example3() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day12/example3"));
        assert_eq!(part1_answer, 1930);
        assert_eq!(part2_answer, 1206);
    }
}
