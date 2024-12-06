use std::cmp::Ordering;
use std::collections::BTreeSet;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

#[derive(Default, Debug)]
struct Puzzle {
    rules: BTreeSet<[usize; 2]>,
    updates: Vec<Vec<usize>>
}

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let puzzle = parse_puzzle(input);
    puzzle.updates.iter()
        .filter(|&u| is_sorted(&puzzle.rules, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn parse_puzzle(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();

    for line in input.lines() {
        if line.contains("|") {
            let mut num_it = line.trim()
                    .split("|")
                    .map(|s| s.parse().unwrap());
            puzzle.rules.insert([num_it.next().unwrap(), num_it.next().unwrap()]);
        } else if line.contains(",") {
            puzzle.updates.push(
                line.trim()
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect()
            );
        }
    }

    puzzle
}

fn is_sorted(rules: &BTreeSet<[usize; 2]>, update: &Vec<usize>) -> bool {
    for i in 0..update.len()-1 {
        for j in i+1..update.len() {
            if rules.contains(&[update[j], update[i]]) {
                return false;
            }
        }
    }    

    true
}

fn solve_part_2(input: &str) -> usize {
    let puzzle = parse_puzzle(input);
    puzzle.updates.into_iter()
        .filter(|u| !is_sorted(&puzzle.rules, u))
        .map(|mut u| {
            u.sort_by(|&a, &b| if puzzle.rules.contains(&[a,b]) {
                Ordering::Less
            } else if puzzle.rules.contains(&[b,a]) {
                Ordering::Greater
            } else {
                Ordering::Equal
            });

            u[u.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_is_sorted() {
        let puzzle = parse_puzzle(TEST_INPUT);

        assert_eq!(
            puzzle.updates.iter()
                .map(|u| is_sorted(&puzzle.rules, u))
                .collect::<Vec<_>>(),
            vec![true, true, true, false, false, false]
        )
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 143)
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 123)
    }
}
