use adventofcode_2024::graph::maximal_cliques;
use adventofcode_2024::{graph, print_answers};
use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;

/// https://adventofcode.com/2024/day/23
fn main() {
    print_answers(run(include_str!("../../input/day23/input")));
}

fn run(input: &'static str) -> (usize, String) {
    let edges = input.lines().map(|l| l.split_once('-').unwrap());
    let g = UnGraphMap::<&str, ()>::from_edges(edges);

    let three_cliques = graph::three_cliques(&g);
    let part1_answer = three_cliques
        .iter()
        .filter(|c| c.iter().any(|&n| n.starts_with('t')))
        .count();

    let max_cliques = maximal_cliques(&g);
    let part2_answer = max_cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",");

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day23/input"));
        assert_eq!(part1_answer, 1149);
        assert_eq!(part2_answer, "as,co,do,kh,km,mc,np,nt,un,uq,wc,wz,yo");
    }

    #[test]
    fn example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day23/example"));
        assert_eq!(part1_answer, 7);
        assert_eq!(part2_answer, "co,de,ka,ta");
    }
}
