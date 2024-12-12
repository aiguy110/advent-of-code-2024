#![feature(let_chains)]
use std::collections::BTreeSet;
use grid::{Grid, GridVec};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut total_score = 0;
    for i in 0..grid.row_count {
        for j in 0..grid.col_count {
            let loc = GridVec::from([i, j]);
            if grid[loc] == 0 {
                total_score += peaks_reachable_from(loc, &grid).len();
            }
        }
    }

    total_score
}

fn solve_part_2(input: &str) -> u32 {
    let grid = parse_input(input);
    let mut total_rating = 0;
    for i in 0..grid.row_count {
        for j in 0..grid.col_count {
            let loc = GridVec::from([i, j]);
            if grid[loc] == 0 {
                total_rating += trail_count_from(loc, &grid);
            }
        }
    }

    total_rating
}

fn parse_input(input: &str) -> Grid<u8> {
    Grid::from_iter(
        input.lines()
            .map(|l| l.trim()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
            )
    ).unwrap()
}

fn peaks_reachable_from(start: GridVec, grid: &Grid<u8>) -> BTreeSet<GridVec> {
    if let Some(level) = grid.get(start) && *level == 9 {
        return [start].into_iter().collect()
    }

    let mut adj_dirs = vec![GridVec::from([1, 0])];
    for _ in 0..3 {
        adj_dirs.push(adj_dirs[adj_dirs.len()-1].rot_90());
    }

    let this_level = grid[start];
    adj_dirs.into_iter()
        .map(|dir| start + dir)
        .filter(|&adj_loc| match grid.get(adj_loc) { Some(&adj_level) => adj_level == this_level + 1, None => false})
        .map(|loc| peaks_reachable_from(loc, grid))
        .flatten()
        .collect()
}

fn trail_count_from(start: GridVec, grid: &Grid<u8>) -> u32 {
    if let Some(level) = grid.get(start) && *level == 9 {
        return 1;
    }

    let mut adj_dirs = vec![GridVec::from([1, 0])];
    for _ in 0..3 {
        adj_dirs.push(adj_dirs.last().unwrap().rot_90());
    }

    let this_level = grid[start];
    let res = adj_dirs.into_iter()
        .map(|dir| start + dir)
        .filter(|&adj_loc| match grid.get(adj_loc) { Some(&adj_level) => adj_level == this_level + 1, None => false})
        .map(|loc| trail_count_from(loc, grid))
        .sum();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 36);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 81);
    }
}
