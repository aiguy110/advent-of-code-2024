use std::collections::BTreeMap;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve(PUZZLE_INPUT, 25));
    println!("Part 2 Answer: {}", solve(PUZZLE_INPUT, 75));
}

fn solve(input: &str, iterations: usize) -> usize {
    let mut state = parse_input(input);
    for _ in 0..iterations {
        state = tick(state);
    }

    state.values().sum()
}

fn parse_input(input: &str) -> BTreeMap<usize, usize> {
    let mut stone_counts = BTreeMap::new();

    for n in input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap()) 
    {
        if let Some(c) = stone_counts.get_mut(&n) {
            *c += 1;
        } else {
            stone_counts.insert(n, 1);
        }
    }

    stone_counts
}

fn tick(state: BTreeMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut new_state = BTreeMap::new(); 

    for num_count in state.into_iter() {
        match num_count {
            (0, c) => {
                add_or_set(1, c, &mut new_state);
            },
            (n, c) if n.to_string().len() % 2 == 0 => {
                let n_str = n.to_string();
                add_or_set(n_str[..n_str.len()/2].parse().unwrap(), c, &mut new_state);
                add_or_set(n_str[n_str.len()/2..].parse().unwrap(), c, &mut new_state);
            },
            (n, c) => {
                add_or_set(2024*n, c, &mut new_state);
            }
        }
    }

    new_state
}

fn add_or_set(key: usize, val: usize, dict: &mut BTreeMap<usize, usize>) {
    if let Some(v) = dict.get_mut(&key) {
        *v += val;
    } else {
        dict.insert(key, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve(TEST_INPUT, 25), 55312);
    }

    // #[test]
    // fn test_solve_part_2() {
    //     assert_eq!(solve(TEST_INPUT, 75), 42);
    // }
}
