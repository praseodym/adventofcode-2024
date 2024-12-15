use adventofcode_2024::grid::parse_char_grid_nonsquare;
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

    let (grid, grid_dim) = parse_char_grid_nonsquare(grid_input);
    let part1_answer = simulate(grid, &commands, grid_dim);

    let mut expanded_grid = grid_input.to_string();
    expanded_grid = expanded_grid.replace("#", "##");
    expanded_grid = expanded_grid.replace("O", "[]");
    expanded_grid = expanded_grid.replace(".", "..");
    expanded_grid = expanded_grid.replace("@", "@.");

    let (grid, grid_dim) = parse_char_grid_nonsquare(&expanded_grid);
    let part2_answer = simulate(grid, &commands, grid_dim);

    (part1_answer, part2_answer)
}

fn simulate(mut grid: Vec<u8>, commands: &[u8], (width, height): (usize, usize)) -> u32 {
    let mut robot = grid
        .iter()
        .enumerate()
        .position(|(_, &c)| c == b'@')
        .map(|i| (i % width, i / width))
        .unwrap();

    for c in commands {
        let (x, y) = robot;
        let (dx, dy) = match c {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            _ => unreachable!(),
        };
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
            continue;
        }
        let nc = grid[ny as usize * width + nx as usize];
        match nc {
            b'#' => continue,
            b'O' => {
                if !push_boxes_part1(&mut grid, width, height, nx, ny, dx, dy) {
                    continue;
                }
            }
            b'[' => {
                if !push_boxes_part2(&mut grid, width, height, nx, ny, dx, dy) {
                    continue;
                }
            }
            b']' => {
                if !push_boxes_part2(&mut grid, width, height, nx - 1, ny, dx, dy) {
                    continue;
                }
            }
            b'.' => {}
            _ => unreachable!(),
        }
        grid[y * width + x] = b'.';
        grid[ny as usize * width + nx as usize] = b'@';
        robot = (nx as usize, ny as usize);
    }

    grid.iter()
        .enumerate()
        .filter(|(_, &c)| c == b'O' || c == b'[')
        .map(|(i, _)| 100 * (i / width) as u32 + (i % width) as u32)
        .sum::<u32>()
}

fn push_boxes_part1(
    grid: &mut [u8],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) -> bool {
    assert_eq!(grid[y as usize * width + x as usize], b'O');
    let (bx, by) = (x + dx, y + dy);
    if bx < 0 || by < 0 || bx >= width as i32 || by >= height as i32 {
        return false;
    }
    let bc = grid[by as usize * width + bx as usize];
    if bc == b'#' {
        return false;
    }
    if bc == b'O' && !push_boxes_part1(grid, width, height, bx, by, dx, dy) {
        return false;
    }
    grid[y as usize * width + x as usize] = b'.';
    grid[by as usize * width + bx as usize] = b'O';
    true
}

fn push_boxes_part2(
    grid: &mut [u8],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) -> bool {
    let mut to_move = Vec::new();
    let mut to_check = VecDeque::new();
    to_check.push_back((x, y));
    while let Some((x, y)) = to_check.pop_front() {
        if to_move.contains(&(x, y)) {
            continue;
        }
        for (bx, by) in [(x + dx, y + dy), (x + dx + 1, y + dy)] {
            if bx < 0 || by < 0 || bx >= width as i32 || by >= height as i32 {
                return false;
            }
            let bc = grid[by as usize * width + bx as usize];
            if bc == b'#' {
                return false;
            }
            if bc == b'[' {
                to_check.push_back((bx, by));
            } else if bc == b']' {
                to_check.push_back((bx - 1, by));
            }
        }
        to_move.push((x, y));
    }

    while let Some((x, y)) = to_move.pop() {
        let (bx, by) = (x + dx, y + dy);
        grid[y as usize * width + x as usize] = b'.';
        grid[y as usize * width + x as usize + 1] = b'.';
        grid[by as usize * width + bx as usize] = b'[';
        grid[by as usize * width + bx as usize + 1] = b']';
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
