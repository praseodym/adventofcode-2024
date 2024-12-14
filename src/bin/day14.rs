use adventofcode_2024::print_answers;

/// https://adventofcode.com/2024/day/14
fn main() {
    print_answers(run(include_str!("../../input/day14/input"), (101, 103)));
}

fn run(input: &'static str, (width, height): (u32, u32)) -> (u32, u32) {
    let mut part1_answer = 0;
    let mut part2_answer;

    let re = regex::Regex::new("p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let mut robots = vec![];
    for line in input.lines() {
        let c = re.captures(line).unwrap();
        let x = c[1].parse::<i32>().unwrap();
        let y = c[2].parse::<i32>().unwrap();
        let vx = c[3].parse::<i32>().unwrap();
        let vy = c[4].parse::<i32>().unwrap();

        robots.push(((x, y), (vx, vy)));
    }

    let width = width as i32;
    let height = height as i32;

    let mut i = 0;
    'outer: loop {
        i += 1;

        for ((x, y), (vx, vy)) in &mut robots {
            *x += *vx;
            *y += *vy;
            *x = x.rem_euclid(width);
            *y = y.rem_euclid(height);
        }

        if i == 100 {
            part1_answer = safety_factor(&robots, (width, height));
        }

        for i in 0..robots.len() {
            for j in i + 1..robots.len() {
                if robots[i].0 == robots[j].0 {
                    continue 'outer;
                }
            }
        }

        part2_answer = i;

        for y in 0..height {
            for x in 0..width {
                if robots.iter().any(|((rx, ry), _)| *rx == x && *ry == y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();

        if i > 100 {
            break;
        }
    }

    (part1_answer, part2_answer)
}

type Robot = ((i32, i32), (i32, i32));
fn safety_factor(robots: &Vec<Robot>, (width, height): (i32, i32)) -> u32 {
    let mut quadrants = [0; 4];
    for ((x, y), _) in robots {
        if *x < width / 2 && *y < height / 2 {
            quadrants[0] += 1;
        } else if *x > width / 2 && *y < height / 2 {
            quadrants[1] += 1;
        } else if *x < width / 2 && *y > height / 2 {
            quadrants[2] += 1;
        } else if *x > width / 2 && *y > height / 2 {
            quadrants[3] += 1;
        }
    }
    quadrants.iter().product()
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn input() {
        let (part1_answer, part2_answer) = run(include_str!("../../input/day14/input"), (101, 103));
        assert_eq!(part1_answer, 236628054);
        assert_eq!(part2_answer, 7584);
    }

    #[test]
    fn example() {
        let (part1_answer, _) = run(include_str!("../../input/day14/example"), (7, 11));
        assert_eq!(part1_answer, 12);
    }
}
