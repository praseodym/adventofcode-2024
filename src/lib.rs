use std::fmt::Display;

pub mod grid;

pub fn print_answers<T: Display, U: Display>((part1_answer, part2_answer): (T, U)) {
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}
