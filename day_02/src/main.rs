#![feature(iter_map_windows)]

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = vec![];
    for line in input.lines() {
        let mut report = vec![];
        for level in line.split_whitespace().map(|s| s.parse().unwrap()) {
            report.push(level);
        }
        reports.push(report);
    }

    reports
}

fn report_is_safe_v1(report: &Vec<i32>) -> bool {
    if report[0] < report[1] { // Ascending report
         report.iter()
             .map_windows(|[&n1, &n2]| n2-n1)
             .all(|dif| dif >= 1 && dif <= 3)
    } else if report[0] > report[1] { // Descending report
        report.iter()
            .map_windows(|[&n1, &n2]| n1-n2)
            .all(|dif| dif >= 1 && dif <= 3)
    } else {
        false
    }
}

fn report_is_safe_v2(report: &Vec<i32>) -> bool {
    let mut is_safe = false;

    if report[0] < report[1] { // Ascending report
         is_safe = report.iter()
             .map_windows(|[&n1, &n2]| n2-n1)
             .all(|dif| dif >= 1 && dif <= 3)
    } else if report[0] > report[1] { // Descending report
        is_safe = report.iter()
            .map_windows(|[&n1, &n2]| n1-n2)
            .all(|dif| dif >= 1 && dif <= 3)
    } 

    if !is_safe {
        is_safe = (0..report.len())
            .into_iter()
            .any(|i| report_is_safe_with_skip(report, i))
    }

    is_safe
}

fn report_is_safe_with_skip(report: &Vec<i32>, skip_ind: usize) -> bool {
    let report = report.iter()
        .enumerate()
        .filter_map(|(i, level)| if i == skip_ind { None } else {Some(level)})
        .collect::<Vec<_>>();
        
    if report[0] < report[1] { // Ascending report
         report.iter()
             .map_windows(|[&n1, &n2]| n2-n1)
             .all(|dif| dif >= 1 && dif <= 3)
    } else if report[0] > report[1] { // Descending report
        report.iter()
            .map_windows(|[&n1, &n2]| n1-n2)
            .all(|dif| dif >= 1 && dif <= 3)
    } else {
        false
    }
}

fn solve_part_1(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter()
        .filter(|report| report_is_safe_v1(report))
        .count()
}

fn solve_part_2(reports: Vec<Vec<i32>>) -> usize {
    reports.iter()
        .map(|report| report_is_safe_v2(report))
        .filter(|&safe| safe)
        .count()
}

fn main() {
    let reports = parse_input(PUZZLE_INPUT);
    println!("Part 1 Answer: {}", solve_part_1(&reports));
    println!("Part 2 Answer: {}", solve_part_2(reports));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_parse_input() {
        let parsed_reports = parse_input(TEST_INPUT);
        let target_reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(parsed_reports, target_reports);
    }

    #[test]
    fn test_report_is_safe_v1() {
        let reports = parse_input(TEST_INPUT);

        assert_eq!(reports.iter()
                   .map(|r| report_is_safe_v1(r))
                   .collect::<Vec<_>>(),
                   vec![true, false, false, false, false, true])
        
    }

    #[test]
    fn test_solve_part_1() {
        let reports = parse_input(TEST_INPUT);
        assert_eq!(solve_part_1(&reports), 2)
    }

    #[test]
    fn test_report_is_safe_v2() {
        let reports = parse_input(TEST_INPUT);

        assert_eq!(reports.iter()
                   .map(|r| report_is_safe_v2(r))
                   .collect::<Vec<_>>(),
                   vec![true, false, false, true, true, true])
        
    }

    #[test]
    fn test_solve_part_2() {
        let reports = parse_input(TEST_INPUT);
        assert_eq!(solve_part_1(&reports), 2)
    }
}




