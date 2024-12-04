/// https://adventofcode.com/2024/day/4
fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../../input/day04/input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u32, u32) {
    let (grid, len) = parse_char_grid(input);

    let mut part1_answer = 0;
    for x in 0..len {
        for y in 0..len {
            if grid[y][x] == 'X' {
                let horizontal = x + 3 < len
                    && grid[y][x + 1] == 'M'
                    && grid[y][x + 2] == 'A'
                    && grid[y][x + 3] == 'S';
                let horizontal_backwards = x >= 3
                    && grid[y][x - 1] == 'M'
                    && grid[y][x - 2] == 'A'
                    && grid[y][x - 3] == 'S';
                let vertical = y + 3 < len
                    && grid[y + 1][x] == 'M'
                    && grid[y + 2][x] == 'A'
                    && grid[y + 3][x] == 'S';
                let vertical_backwards = y >= 3
                    && grid[y - 1][x] == 'M'
                    && grid[y - 2][x] == 'A'
                    && grid[y - 3][x] == 'S';
                let diagonal_se = x + 3 < len
                    && y + 3 < len
                    && grid[y + 1][x + 1] == 'M'
                    && grid[y + 2][x + 2] == 'A'
                    && grid[y + 3][x + 3] == 'S';
                let diagonal_sw = x >= 3
                    && y + 3 < len
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y + 2][x - 2] == 'A'
                    && grid[y + 3][x - 3] == 'S';
                let diagonal_nw = x >= 3
                    && y >= 3
                    && grid[y - 1][x - 1] == 'M'
                    && grid[y - 2][x - 2] == 'A'
                    && grid[y - 3][x - 3] == 'S';
                let diagonal_ne = x + 3 < len
                    && y >= 3
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y - 2][x + 2] == 'A'
                    && grid[y - 3][x + 3] == 'S';

                part1_answer += (horizontal as u32)
                    + (horizontal_backwards as u32)
                    + (vertical as u32)
                    + (vertical_backwards as u32)
                    + (diagonal_se as u32)
                    + (diagonal_sw as u32)
                    + (diagonal_nw as u32)
                    + (diagonal_ne as u32);
            }
        }
    }

    let mut part2_answer = 0;
    for x in 0..len {
        for y in 0..len {
            if grid[y][x] == 'A' {
                let mas_mas = x >= 1
                    && x + 1 < len
                    && y >= 1
                    && y + 1 < len
                    && grid[y - 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'S'
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y + 1][x + 1] == 'S';
                let sam_sam = x >= 1
                    && x + 1 < len
                    && y >= 1
                    && y + 1 < len
                    && grid[y - 1][x - 1] == 'S'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y + 1][x + 1] == 'M';
                let sam_mas = x >= 1
                    && x + 1 < len
                    && y >= 1
                    && y + 1 < len
                    && grid[y - 1][x - 1] == 'S'
                    && grid[y - 1][x + 1] == 'S'
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y + 1][x + 1] == 'M';
                let mas_sam = x >= 1
                    && x + 1 < len
                    && y >= 1
                    && y + 1 < len
                    && grid[y - 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y + 1][x + 1] == 'S';

                part2_answer +=
                    (mas_mas as u32) + (sam_sam as u32) + (sam_mas as u32) + (mas_sam as u32);
            }
        }
    }

    (part1_answer, part2_answer)
}

fn parse_char_grid(input: &str) -> (Vec<Vec<char>>, usize) {
    let len = input.lines().next().unwrap().len();
    assert_eq!(len, input.lines().count(), "input must be square");
    let mut grid = vec![vec![' '; len]; len];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid[y][x] = c;
        });
    });
    (grid, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day04/input"));
        assert_eq!(part1_answer, 2545);
        assert_eq!(part2_answer, 1886);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day04/example"));
        assert_eq!(part1_answer, 18);
        assert_eq!(part2_answer, 9);
    }
}
