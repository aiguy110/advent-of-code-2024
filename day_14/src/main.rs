use std::collections::BTreeSet;

use grid::GridVec;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT, &[103, 101]));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT, &[103, 101]));
}

struct Bot {
    loc: GridVec,
    vel: GridVec
}

fn solve_part_1(input: &str, dims: &[i64; 2]) -> usize {
    let mut bots = parse_input(input);
    for t in 0..100 {
        tick(&mut bots, dims);
    }
    calc_safety_factor(&bots, dims)
}

fn solve_part_2(input: &str, dims: &[i64; 2]) -> usize {
    let mut bots = parse_input(input);
    let mut t = 0;
    loop {
        if bots_are_clustered(&bots) {
            println!("Time Step: {}", t);
            render_bots(&bots, dims);
            break;
        }

        tick(&mut bots, dims);
        t += 1;
    }

    t
}

fn parse_input(input: &str) -> Vec<Bot> {
    let p = regex::Regex::new(r"p=(\-?\d+),(\-?\d+) v=(\-?\d+),(\-?\d+)").unwrap();
    input.lines()
        .map(|l| p.captures(l).unwrap())
        .map(|caps| Bot {
            loc: GridVec {
                i: caps.get(2).unwrap().as_str().parse().unwrap(),
                j: caps.get(1).unwrap().as_str().parse().unwrap(),
            },
            vel: GridVec {
                i: caps.get(4).unwrap().as_str().parse().unwrap(),
                j: caps.get(3).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn tick(bots: &mut Vec<Bot>, dims: &[i64; 2]) {
    for bot in bots.iter_mut() {
        bot.loc += bot.vel;
        bot.loc.i = true_mod(bot.loc.i, dims[0]);
        bot.loc.j = true_mod(bot.loc.j, dims[1]);
    }
}

fn true_mod(n: i64, m: i64) -> i64 {
    ((n % m) + m) % m
}

fn calc_safety_factor(bots: &Vec<Bot>, dims: &[i64; 2]) -> usize {
    let mut quad_counts = [0, 0, 0, 0];
    for bot in bots {
        if bot.loc.i == dims[0]/2 || bot.loc.j == dims[1]/2 {
            continue;
        }
        let quad = (bot.loc.i/(dims[0]/2+1))*2 + bot.loc.j / (dims[1]/2+1);
        quad_counts[quad as usize] += 1;
    }

    quad_counts.into_iter().product()
}

fn render_bots(bots: &Vec<Bot>, dims: &[i64; 2]) {
    let bot_locs = BTreeSet::from_iter(bots.iter().map(|b| b.loc));
    for i in 0..dims[0] {
        for j in 0..dims[1] {
            if bot_locs.contains(&GridVec::from([i, j])) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn bots_are_clustered(bots: &Vec<Bot>) -> bool {
    let adj_dirs = (-1..=1)
        .map(|i| (-1..=1).map(move |j| GridVec::from([i, j])))
        .flatten()
        .filter(|v| v.i != 0 || v.j != 0)
        .collect::<Vec<_>>();

    let mut outlier_count = 0;
    let bot_locs = BTreeSet::from_iter(bots.iter().map(|b| b.loc));
    for bot_loc in bot_locs.iter() {
        if ! adj_dirs.iter()
            .map(|dir| *bot_loc + *dir)
            .any(|loc| bot_locs.contains(&loc))
        {
            outlier_count += 1;
        }
    }

    outlier_count < bot_locs.len() / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT, &[7, 11]), 12);
    }
}
