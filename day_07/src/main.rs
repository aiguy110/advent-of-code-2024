const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let eqs = parse_puzzle(input);
    eqs.iter()
        .filter(|eq| can_eq(eq.0, eq.1[0], &eq.1[1..]))
        .map(|eq| eq.0)
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let eqs = parse_puzzle(input);
    eqs.iter()
        .filter(|eq| can_eq_2(eq.0, eq.1[0], &eq.1[1..]))
        .map(|eq| eq.0)
        .sum()
}

fn parse_puzzle(input: &str) -> Vec<(usize, Vec<usize>)> {
    input.lines()
        .map(|l| l.trim().split(':'))
        .map(|mut it| 
             (
                 it.next().unwrap().parse().unwrap(),
                 it.next().unwrap().split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
        ))
        .collect()
}

fn can_eq(target: usize, accum: usize, remaining: &[usize]) -> bool {
    if remaining.len() == 0 {
        return accum == target
    }

    can_eq(target, accum + remaining[0], &remaining[1..]) || can_eq(target, accum * remaining[0], &remaining[1..])
}

fn can_eq_2(target: usize, accum: usize, remaining: &[usize]) -> bool {
    if remaining.len() == 0 {
        return accum == target
    }

    can_eq_2(target, accum + remaining[0], &remaining[1..]) 
    || can_eq_2(target, accum * remaining[0], &remaining[1..])
    || can_eq_2(target, cat_nums(accum, remaining[0]), &remaining[1..])
}

fn cat_nums(a: usize, b:usize) -> usize {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_part_1_hard_negative() {
        // I originally kicked off the recusion with `can_eq(eq.0, 0, &eq.1[..])`
        // because I didn't think of this case where zeroing the first term can
        // make the equation solvable when it should not be.
        assert_eq!(solve_part_1("1: 10 1"), 0);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 11387);
    }

    #[test]
    fn test_cat_nums() {
        assert_eq!(cat_nums(1,2), 12);
    }
}
