use adventofcode_2024::print_answers;
use std::collections::VecDeque;

/// https://adventofcode.com/2024/day/9
fn main() {
    print_answers(run(include_str!("../../input/day09/input")));
}

#[derive(Clone, Debug)]
struct Block {
    offset: u64,
    id: u64,
    size: u64,
}

fn run(input: &'static str) -> (u64, u64) {
    let (blocks, files, free_blocks) = parse(input);

    let part1_answer = solve_part1(blocks.clone());
    let part2_answer = solve_part2(blocks, files, free_blocks);

    (part1_answer, part2_answer)
}

#[allow(clippy::mut_range_bound)]
fn solve_part1(mut blocks: VecDeque<Option<u64>>) -> u64 {
    let mut first_free = 0;
    'outer: loop {
        for i in first_free..blocks.len() {
            if blocks[i].is_none() {
                let block = blocks.pop_back().unwrap();
                blocks[i] = block;
                first_free = i;
                continue 'outer;
            }
        }
        break;
    }

    checksum(&blocks)
}

fn solve_part2(
    mut blocks: VecDeque<Option<u64>>,
    files: Vec<Block>,
    mut free_blocks: Vec<Block>,
) -> u64 {
    for file in files.iter().rev() {
        for free_block in free_blocks.iter_mut() {
            if free_block.id >= file.id {
                break;
            }
            if free_block.size >= file.size {
                (0..file.size).for_each(|j| {
                    blocks[(free_block.offset + j) as usize] = Some(file.id);
                    blocks[(file.offset + j) as usize] = None;
                });
                free_block.offset += file.size;
                free_block.size -= file.size;
                break;
            }
        }
    }

    checksum(&blocks)
}

fn checksum(blocks: &VecDeque<Option<u64>>) -> u64 {
    let part2_answer = blocks
        .iter()
        .enumerate()
        .map(|(i, block)| block.unwrap_or(0) * (i as u64))
        .sum();
    part2_answer
}

fn parse(input: &str) -> (VecDeque<Option<u64>>, Vec<Block>, Vec<Block>) {
    let line = input.trim_ascii();

    let mut blocks: VecDeque<Option<u64>> = VecDeque::new();
    let mut files: Vec<Block> = vec![];
    let mut free_blocks: Vec<Block> = vec![];

    for (i, c) in line.chars().enumerate() {
        let c = c.to_digit(10).unwrap();
        let block = Block {
            offset: blocks.len() as u64,
            id: i as u64 / 2,
            size: c as u64,
        };
        let content = if i % 2 == 0 {
            files.push(block);
            Some((i / 2) as u64)
        } else {
            free_blocks.push(block);
            None
        };
        (0..c).for_each(|_| {
            blocks.push_back(content);
        });
    }
    (blocks, files, free_blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day09/input"));
        assert_eq!(part1_answer, 6330095022244);
        assert_eq!(part2_answer, 6359491814941);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day09/example"));
        assert_eq!(part1_answer, 1928);
        assert_eq!(part2_answer, 2858);
    }
}
