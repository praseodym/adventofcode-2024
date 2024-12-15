use adventofcode_2024::grid::Grid;
use adventofcode_2024::point::{Point, LEFT, RIGHT};
use adventofcode_2024::print_answers;
use std::collections::VecDeque;

/// https://adventofcode.com/2024/day/15
fn main() {
    print_answers(run(include_str!("../../input/day15/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let input = input.split_once("\n\n").unwrap();
    let grid_input = input.0;
    let commands = input.1.replace("\n", "").bytes().collect::<Vec<u8>>();
    let grid = Grid::parse_square(grid_input);

    let part1_answer = simulate(grid, &commands);

    let mut expanded_grid = grid_input.to_string();
    expanded_grid = expanded_grid.replace("#", "##");
    expanded_grid = expanded_grid.replace("O", "[]");
    expanded_grid = expanded_grid.replace(".", "..");
    expanded_grid = expanded_grid.replace("@", "@.");

    let grid = Grid::parse_nonsquare(&expanded_grid);
    let part2_answer = simulate(grid, &commands);

    (part1_answer, part2_answer)
}

fn simulate(mut grid: Grid<u8>, commands: &[u8]) -> u32 {
    let mut robot = grid.position(|c| *c == b'@').unwrap();

    for c in commands {
        let dir = Point::from_ascii(*c);
        let new = robot + dir;
        if !grid.contains(new) {
            continue;
        }
        let nc = grid[new];
        match nc {
            b'#' => continue,
            b'O' => {
                if !push_boxes_part1(&mut grid, new, dir) {
                    continue;
                }
            }
            b'[' => {
                if !push_boxes_part2(&mut grid, new, dir) {
                    continue;
                }
            }
            b']' => {
                if !push_boxes_part2(&mut grid, new + LEFT, dir) {
                    continue;
                }
            }
            b'.' => {}
            _ => unreachable!(),
        }
        grid[robot] = b'.';
        grid[new] = b'@';
        robot = new;
    }

    grid.data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'O' || c == b'[')
        .map(|(i, _)| grid.point_from_index(i))
        .map(|p| 100 * p.y as u32 + p.x as u32)
        .sum::<u32>()
}

fn push_boxes_part1(grid: &mut Grid<u8>, pos: Point, dir: Point) -> bool {
    assert_eq!(grid[pos], b'O');
    let b = pos + dir;
    if !grid.contains(b) || grid[b] == b'#' || (grid[b] == b'O' && !push_boxes_part1(grid, b, dir))
    {
        false
    } else {
        grid[b] = b'.';
        grid[b] = b'O';
        true
    }
}

fn push_boxes_part2(grid: &mut Grid<u8>, pos: Point, dir: Point) -> bool {
    let mut to_move = Vec::new();
    let mut to_check = VecDeque::new();
    to_check.push_back(pos);
    while let Some(pos) = to_check.pop_front() {
        if to_move.contains(&pos) {
            continue;
        }
        for &p in [pos + dir, pos + dir + RIGHT].iter() {
            if !grid.contains(p) || grid[p] == b'#' {
                return false;
            }
            if grid[p] == b'[' {
                to_check.push_back(p);
            } else if grid[p] == b']' {
                to_check.push_back(p + LEFT);
            }
        }
        to_move.push(pos);
    }
    while let Some(pos) = to_move.pop() {
        grid[pos] = b'.';
        grid[pos + RIGHT] = b'.';
        let p = pos + dir;
        grid[p] = b'[';
        grid[p + RIGHT] = b']';
    }
    true
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day15/input"));
        assert_eq!(part1_answer, 1451928);
        assert_eq!(part2_answer, 1462788);
    }

    #[test]
    fn example1() {
        let (part1_answer, _) = run(include_str!("../../input/day15/example1"));
        assert_eq!(part1_answer, 2028);
    }

    #[test]
    fn example2() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day15/example2"));
        assert_eq!(part1_answer, 10092);
        assert_eq!(part2_answer, 9021);
    }

    #[test]
    fn example3() {
        run(include_str!("../../input/day15/example3"));
    }
}
