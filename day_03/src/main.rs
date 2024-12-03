const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn solve_part_1(input: &str) -> i32 {
    let pattern = regex::Regex::new(r"(?s)mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for cap in pattern.captures_iter(input) {
        let n1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let n2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        total += n1*n2;
    }

    total
}

fn solve_part_2(input: &str) -> i32 {
    let pattern = regex::Regex::new(r"(?s)((do\(\))|(don't\(\))|mul\((\d+),(\d+)\))").unwrap();
    let mut total = 0;
    let mut should_do = true;
    for cap in pattern.captures_iter(input) {
        if let Some(_) = cap.get(2) {
            should_do = true;
        } else if let Some(_) = cap.get(3) {
            should_do = false;
        } else if should_do {
            let n1 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let n2 = cap.get(5).unwrap().as_str().parse::<i32>().unwrap();
            total += n1*n2;
        }
    }

    total
}

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT_1), 161)
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT_2), 48)
    }
}
