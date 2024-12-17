const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

use grid::GridVec;
use regex::Regex;

struct ClawMachine {
    a_step: GridVec,
    b_step: GridVec,
    target_loc: GridVec
}

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> u64 {
    let machines = parse_input(input);

    machines.iter()
        .filter_map(|m| solve_machine(m))
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let target_offset = GridVec::from([10000000000000i64, 10000000000000i64]);
    let mut machines = parse_input(input);
    for machine in machines.iter_mut() {
        machine.target_loc += target_offset;
    }
    
    machines.iter()
        .filter_map(|m| solve_machine(m))
        .sum()
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    let button_pattern = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let mut machines = Vec::new();
    let mut lines = input.lines().map(|l| l.trim());
    loop {
        let a_line = lines.next().unwrap();
        let caps = button_pattern.captures(a_line).unwrap();
        let a_step = GridVec {
            i: caps.get(2).unwrap().as_str().parse().unwrap(),
            j: caps.get(1).unwrap().as_str().parse().unwrap()
        };

        let b_line = lines.next().unwrap();
        let caps = button_pattern.captures(b_line).unwrap();
        let b_step = GridVec {
            i: caps.get(2).unwrap().as_str().parse().unwrap(),
            j: caps.get(1).unwrap().as_str().parse().unwrap()
        };

        let target_line = lines.next().unwrap();
        let caps = prize_pattern.captures(target_line).unwrap();
        let target_loc = GridVec {
            i: caps.get(2).unwrap().as_str().parse().unwrap(),
            j: caps.get(1).unwrap().as_str().parse().unwrap()
        };

        machines.push( ClawMachine {
            a_step,
            b_step,
            target_loc
        });

        if matches!(lines.next(), None) {
            break;
        }
    }

    machines
}

fn solve_machine(machine: &ClawMachine) -> Option<u64> {
    let &ClawMachine {
        a_step: a,
        b_step: b,
        target_loc: t
    } = machine;

    let det = a.i*b.j - a.j*b.i;
    let a_count = (t.i*b.j - t.j*b.i) / det; 
    let b_count = (a.i*t.j - a.j*t.i) / det;

    if a_count*a + b_count*b == t {
        Some((3*a_count + b_count) as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 480);
    }
}
