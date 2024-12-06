/// https://adventofcode.com/2024/day/6
fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../../input/day06/input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let (grid, len) = parse_char_grid(input);

    let start = grid.iter().position(|&c| c == b'^').unwrap();
    let start = (start % len, start / len);

    let (part1_answer, visited) = patrol(&grid, len, start, None);
    let part1_answer = part1_answer.unwrap();

    let mut part2_answer = 0;
    for y in 0..len {
        for x in 0..len {
            if visited[y * len + x] && patrol(&grid, len, start, Some((x, y))).0.is_none() {
                part2_answer += 1;
            }
        }
    }

    (part1_answer, part2_answer)
}

fn patrol(
    grid: &[u8],
    len: usize,
    start: (usize, usize),
    obstacle: Option<(usize, usize)>,
) -> (Option<u32>, Vec<bool>) {
    let mut pos = start;
    let mut dir = (0, -1);

    let mut visited = vec![false; len * len];
    let mut visited_dir = vec![(0isize, 0isize); len * len];

    visited[start.1 * len + start.0] = true;

    loop {
        let new_pos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if new_pos.0 < 0 || new_pos.0 >= len as isize || new_pos.1 < 0 || new_pos.1 >= len as isize
        {
            return (Some(visited.iter().filter(|&&v| v).count() as u32), visited);
        }

        if grid[new_pos.1 as usize * len + new_pos.0 as usize] == b'#'
            || Some((new_pos.0 as usize, new_pos.1 as usize)) == obstacle
        {
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            };
        } else {
            pos = (new_pos.0 as usize, new_pos.1 as usize);

            if visited[pos.1 * len + pos.0] && visited_dir[pos.1 * len + pos.0] == dir {
                return (None, visited);
            }

            visited[pos.1 * len + pos.0] = true;
            visited_dir[pos.1 * len + pos.0] = dir;
        }
    }
}

fn parse_char_grid(input: &str) -> (Vec<u8>, usize) {
    let len = input.lines().next().unwrap().len();
    assert_eq!(len, input.lines().count(), "input must be square");
    let mut grid = vec![b' '; len * len];
    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, c)| {
            grid[y * len + x] = c;
        });
    });
    (grid, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day06/input"));
        assert_eq!(part1_answer, 4374);
        assert_eq!(part2_answer, 1705);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day06/example"));
        assert_eq!(part1_answer, 41);
        assert_eq!(part2_answer, 6);
    }
}
