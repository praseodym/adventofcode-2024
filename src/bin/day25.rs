use adventofcode_2024::grid::Grid;
use itertools::Itertools;

/// https://adventofcode.com/2024/day/25
fn main() {
    println!(
        "part 1 answer: {}",
        run(include_str!("../../input/day25/input"))
    );
}

fn run(input: &'static str) -> u32 {
    let grids = input
        .split("\n\n")
        .map(Grid::parse_nonsquare)
        .collect::<Vec<_>>();
    grids.iter().tuple_combinations().fold(0, |ans, (g1, g2)| {
        for (p1, p2) in g1.data.iter().zip(g2.data.iter()) {
            if *p1 == b'#' && *p2 == b'#' {
                return ans;
            }
        }
        ans + 1
    })
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn input() {
        let part1_answer = run(include_str!("../../input/day25/input"));
        assert_eq!(part1_answer, 2993);
    }

    #[test]
    fn example() {
        let part1_answer = run(include_str!("../../input/day25/example"));
        assert_eq!(part1_answer, 3);
    }
}
