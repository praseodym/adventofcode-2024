use adventofcode_2024::print_answers;

fn main() {
    print_answers(run(include_str!("../../input/day17/input")));
}

fn run(input: &'static str) -> (String, u64) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut s = registers.split(&[':', '\n'][..]);
    let reg_a = s.nth(1).unwrap().trim_ascii().parse::<u64>().unwrap();
    let reg_b = s.nth(1).unwrap().trim_ascii().parse::<u64>().unwrap();
    let reg_c = s.nth(1).unwrap().trim_ascii().parse::<u64>().unwrap();
    let program: Vec<u64> = program
        .strip_prefix("Program: ")
        .unwrap()
        .trim_ascii()
        .split(',')
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    let part1_answer = run_program(reg_a, reg_b, reg_c, &program)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let part2_answer = find_quine(&program);

    (part1_answer, part2_answer)
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut out = vec![];

    while ip < program.len() {
        let i = program[ip];
        let o = program[ip + 1];
        let co = match o {
            4 => a,
            5 => b,
            6 => c,
            _ => o,
        };
        ip += 2;
        match i {
            // adv
            0 => {
                a /= 2u64.pow(co as u32);
            }
            // bxl
            1 => {
                b ^= o;
            }
            // bst
            2 => {
                b = co % 8;
            }
            // jnz
            3 => {
                if a != 0 {
                    ip = o as usize;
                }
            }
            // bxc
            4 => {
                b ^= c;
            }
            // out
            5 => {
                out.push(co % 8);
            }
            // bdv
            6 => {
                b = a / 2u64.pow(co as u32);
            }
            // cdv
            7 => {
                c = a / 2u64.pow(co as u32);
            }
            _ => unreachable!(),
        }
    }

    out
}

fn find_quine(program: &[u64]) -> u64 {
    let mut stack = vec![(1, 0)];
    while let Some((i, a)) = stack.pop() {
        for a in a..a + 8 {
            let out = run_program(a, 0, 0, program);
            if out == program[program.len() - i..] {
                if i == program.len() {
                    return a;
                }
                stack.push((i + 1, a * 8));
            }
        }
    }
    0
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day17/input"));
        assert_eq!(part1_answer, "1,0,2,0,5,7,2,1,3");
        assert_eq!(part2_answer, 265652340990875);
    }

    #[test]
    fn example() {
        let (part1_answer, _) = run(include_str!("../../input/day17/example1"));
        assert_eq!(part1_answer, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example2() {
        let (_, part2_answer) = run(include_str!("../../input/day17/example2"));
        assert_eq!(part2_answer, 117440);
    }
}
