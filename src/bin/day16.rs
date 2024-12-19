use adventofcode_2024::grid::Grid;
use adventofcode_2024::point::{Point, EAST, NESW};
use adventofcode_2024::print_answers;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

/// https://adventofcode.com/2024/day/16
fn main() {
    print_answers(run(include_str!("../../input/day16/input")));
}

fn run(input: &'static str) -> (u32, u32) {
    let grid = Grid::parse_square(input);

    let start = grid.position(|c| *c == b'S').unwrap();
    let end = grid.position(|c| *c == b'E').unwrap();
    let dir = EAST;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: start,
        dir,
        score: 0,
        visited: vec![start],
    });

    let mut seen = HashSet::new();
    seen.insert((start, dir));
    let mut best_score = u32::MAX;
    let mut best_path_tiles = HashSet::new();

    while let Some(state) = heap.pop() {
        if state.pos == end {
            best_score = best_score.min(state.score);
            if state.score == best_score {
                state.visited.iter().for_each(|p| {
                    best_path_tiles.insert(*p);
                });
            }
        }

        seen.insert((state.pos, state.dir));

        for &new_dir in NESW.iter() {
            let new = state.pos + new_dir;
            if !grid.contains(new) || grid[new] == b'#' || seen.contains(&(new, new_dir)) {
                continue;
            }
            let mut visited = state.visited.clone();
            visited.push(new);
            heap.push(State {
                pos: new,
                dir: new_dir,
                score: state.score + rotation_score(state.dir, new_dir) + 1,
                visited,
            });
        }
    }

    let part1_answer = best_score;
    let part2_answer = best_path_tiles.len() as u32;
    (part1_answer, part2_answer)
}

#[derive(Debug)]
struct State {
    pos: Point,
    dir: Point,
    score: u32,
    visited: Vec<Point>,
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

fn rotation_score(dir: Point, new_dir: Point) -> u32 {
    let mut cur_dir1 = dir;
    let mut cur_dir2 = dir;
    for i in 0..4 {
        if cur_dir1 == new_dir || cur_dir2 == new_dir {
            return i * 1000;
        }
        cur_dir1 = cur_dir1.rotate_90deg_cw();
        cur_dir2 = cur_dir2.rotate_90deg_ccw();
    }
    unreachable!();
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day16/input"));
        assert_eq!(part1_answer, 75416);
        assert_eq!(part2_answer, 476);
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day16/example"));
        assert_eq!(part1_answer, 7036);
        assert_eq!(part2_answer, 45);
    }
}
