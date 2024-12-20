use adventofcode_2024::grid::Grid;
use adventofcode_2024::point::{Point, NESW};
use adventofcode_2024::print_answers;
use std::collections::VecDeque;

/// https://adventofcode.com/2024/day/20
fn main() {
    print_answers(run(include_str!("../../input/day20/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let grid = Grid::parse_square(input);

    let start = grid.position(|&c| c == b'S').unwrap();
    let end = grid.position(|&c| c == b'E').unwrap();

    let dist_start = min_distances(&grid, start);
    let dist_end = min_distances(&grid, end);
    let normal = dist_start[end];

    let part1_answer = cheat_savings(&grid, &dist_start, &dist_end, normal, 2);
    let part2_answer = cheat_savings(&grid, &dist_start, &dist_end, normal, 20);

    (part1_answer, part2_answer)
}

fn cheat_savings(
    grid: &Grid<u8>,
    dist_start: &Grid<usize>,
    dist_end: &Grid<usize>,
    normal: usize,
    max_cheat: i32,
) -> u32 {
    let mut ret = 0;
    for (i, &ds) in dist_start.data.iter().enumerate() {
        let p = grid.point_from_index(i);
        if ds > normal || grid[p] == b'#' {
            continue;
        }

        for x in (p.x - max_cheat)..=(p.x + max_cheat) {
            for y in (p.y - max_cheat)..=(p.y + max_cheat) {
                let q = Point::new(x, y);
                let dc = p.distance(q);
                if !grid.contains(q)
                    || grid[q] == b'#'
                    || dist_end[q] > normal
                    || dc > max_cheat as usize
                {
                    continue;
                }
                let cheat_time = ds + dc + dist_end[q];
                let time_saved = normal.saturating_sub(cheat_time);
                if time_saved >= 100 {
                    ret += 1;
                }
            }
        }
    }
    ret
}

fn min_distances(grid: &Grid<u8>, start: Point) -> Grid<usize> {
    let mut queue = VecDeque::new();
    let mut dist = Grid::new(usize::MAX, grid.width, grid.height);
    queue.push_back(start);
    dist[start] = 0;
    while let Some(pos) = queue.pop_front() {
        for &dir in NESW.iter() {
            let new = pos + dir;
            if !grid.contains(new) || grid[new] == b'#' {
                continue;
            }
            let cost = dist[pos] + 1;
            if dist[new] > cost {
                dist[new] = cost;
                queue.push_back(new);
            }
        }
    }
    dist
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day20/input"));
        assert_eq!(part1_answer, 1389);
        assert_eq!(part2_answer, 1005068);
    }

    #[test]
    fn example() {
        run(include_str!("../../input/day20/example"));
    }
}
