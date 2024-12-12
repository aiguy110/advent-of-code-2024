const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let mut state = parse_input(input);
    for _ in 0..25 {
        state = tick(&state);
    }

    state.len()
}

fn solve_part_2(input: &str) -> usize {
    let mut state = parse_input(input);
    for i in 0..75 {
        println!("Blink #{i}");
        state = tick(&state);
    }

    state.len()
}

fn parse_input(input: &str) -> Vec<usize> {
    input.trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn tick(state: &Vec<usize>) -> Vec<usize> {
    state.iter()
        .map(|n| {
            match n {
                0 => vec![1],
                m if m.to_string().len() % 2 == 0 => {
                    let s = m.to_string();
                    vec![
                        s[..s.len()/2].parse().unwrap(),
                        s[s.len()/2..].parse().unwrap()
                    ]
                },
                m => vec![2024*m]
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 55312);
    }

    // #[test]
    // fn test_solve_part_2() {
    //     assert_eq!(solve_part_2(TEST_INPUT), 42);
    // }
}
