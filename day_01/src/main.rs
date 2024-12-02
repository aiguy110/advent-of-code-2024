use std::collections::BTreeMap;

const PUZZLE_INPUT: &'static str = include_str!("../puzzle_input.txt");

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let mut split = line.split("   ");
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    (list1, list2)
}

fn solve_part_1(mut list1: Vec<usize>, mut list2:Vec<usize>) -> usize {
    list1.sort();
    list2.sort();

    list1.into_iter()
        .zip(list2.into_iter())
        .map(|(n1, n2)| i64::abs(n1 as i64 - n2 as i64) as usize)
        .sum()
}

fn solve_part_2(mut list1: Vec<usize>, mut list2:Vec<usize>) -> usize {
    let mut freqs = BTreeMap::new();
    for n in list2 {
        match freqs.get_mut(&n) {
            Some(v) => *v += 1,
            None => {
                freqs.insert(n, 1);
            }
        }
    }

    list1.into_iter()
        .map(|n| match freqs.get(&n) {
            Some(m) => n*m,
            None => 0
        })
        .sum()
}

fn main() {
    let (list1, list2) = parse_input(PUZZLE_INPUT);
    println!("Part 1 Answer: {}", solve_part_1(list1, list2));

    let (list1, list2) = parse_input(PUZZLE_INPUT);
    println!("Part 2 Answer: {}", solve_part_2(list1, list2));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &'static str = include_str!("../test_input_1.txt");

    #[test]
    fn parse_test_input_1() {
        let (list1, list2) = parse_input(TEST_INPUT_1);
        
        assert_eq!(list1, vec![3,4,2,1,3,3]);
        assert_eq!(list2, vec![4,3,5,3,9,3]);
    }

    #[test]
    fn test_solve_part_1() {
        let (list1, list2) = parse_input(TEST_INPUT_1);

        assert_eq!(solve_part_1(list1, list2), 11);
    }

    #[test]
    fn test_solve_part_2() {
        let (list1, list2) = parse_input(TEST_INPUT_1);

        assert_eq!(solve_part_2(list1, list2), 31);
    }
}
