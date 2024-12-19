use adventofcode_2024::grid::Grid;
use adventofcode_2024::point::{Point, NESW, ORIGIN};
use adventofcode_2024::print_answers;
use std::collections::VecDeque;

/// https://adventofcode.com/2024/day/18
fn main() {
    print_answers(run(
        include_str!("../../input/day18/input"),
        Point::new(70, 70),
        1024,
    ));
}

fn run(input: &'static str, exit: Point, part1_bytes: usize) -> (u32, String) {
    let mut lines = input.lines();
    let mut grid = Grid::new(false, (exit.x + 1) as usize, (exit.y + 1) as usize);

    fn update_grid(line: &str, grid: &mut Grid<bool>) {
        let (a, b) = line.split_once(",").unwrap();
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        grid[Point::new(a, b)] = true;
    }

    (&mut lines)
        .take(part1_bytes)
        .for_each(|line| update_grid(line, &mut grid));

    let part1_answer = shortest_path(&grid, ORIGIN, exit).unwrap() as u32;

    let mut part2_answer = "";
    for line in lines {
        update_grid(line, &mut grid);
        if shortest_path(&grid, ORIGIN, exit).is_none() {
            part2_answer = line;
            break;
        }
    }

    (part1_answer, part2_answer.to_string())
}

fn shortest_path(grid: &Grid<bool>, start: Point, end: Point) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut dist = Grid::new(usize::MAX, 71, 71);
    queue.push_back(start);
    dist[start] = 0;
    while let Some(pos) = queue.pop_front() {
        if pos == end {
            return Some(dist[pos]);
        }

        for &dir in NESW.iter() {
            let new = pos + dir;
            if !grid.contains(new) || grid[new] {
                continue;
            }
            let cost = dist[pos] + 1;
            if dist[new] > cost {
                dist[new] = cost;
                queue.push_back(new);
            }
        }
    }
    None
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(
            include_str!("../../input/day18/input"),
            Point::new(70, 70),
            1024,
        );
        assert_eq!(part1_answer, 372);
        assert_eq!(part2_answer, "25,6");
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(
            include_str!("../../input/day18/example"),
            Point::new(6, 6),
            12,
        );
        assert_eq!(part1_answer, 22);
        assert_eq!(part2_answer, "6,1");
    }
}
