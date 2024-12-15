use adventofcode_2024::grid::Grid;
use adventofcode_2024::point::{Point, DOWN, LEFT, RIGHT, UP};
use std::collections::VecDeque;

/// https://adventofcode.com/2024/day/15
/// The Game
fn main() {
    run(include_str!("../../input/day15/input"));
}

fn run(input: &'static str) {
    let input = input.split_once("\n\n").unwrap();
    let grid_input = input.0;

    let mut expanded_grid = grid_input.to_string();
    expanded_grid = expanded_grid.replace("#", "##");
    expanded_grid = expanded_grid.replace("O", "[]");
    expanded_grid = expanded_grid.replace(".", "..");
    expanded_grid = expanded_grid.replace("@", "@.");

    let grid = Grid::parse_nonsquare(&expanded_grid);
    simulate(grid);
}

fn simulate(mut grid: Grid<u8>) {
    let mut robot = grid.position(|c| *c == b'@').unwrap();

    console::Term::stdout().clear_screen().unwrap();

    loop {
        console::Term::stdout().move_cursor_to(0, 0).unwrap();
        print_grid(&grid.data, (grid.width, grid.height));

        let c = console::Term::stdout().read_char().unwrap();

        let dir = match c {
            'w' => UP,
            'a' => LEFT,
            's' => DOWN,
            'd' => RIGHT,
            _ => continue,
        };
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
}

fn print_grid(grid: &[u8], (width, height): (usize, usize)) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y * width + x] as char);
        }
        println!();
    }
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
