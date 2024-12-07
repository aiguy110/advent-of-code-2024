#![feature(let_chains)]

use grid::*;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn parse_grid(input: &str) -> Grid<char> { 
    Grid::from_iter(input.lines().map(|l| l.trim().chars())).unwrap()
}

fn solve_part_1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let dirs = (-1..=1).into_iter()
        .flat_map(|i| (-1..=1).into_iter().map(move |j| [i, j]))
        .filter(|v| *v != [0, 0])
        .map(|v| GridVec::from(v))
        .collect::<Vec<_>>();

    let mut match_count = 0;
    for i in 0..grid.row_count {
        for j in 0..grid.col_count {
            let offset = GridVec::from([i, j]);
            if grid[offset] == 'X' {
                for &dir in dirs.iter() {
                    if let Some(m) = grid.get(offset + 1*dir) && *m == 'M'
                        && let Some(a) = grid.get(offset + 2*dir) && *a == 'A'
                        && let Some(s) = grid.get(offset + 3*dir) && *s == 'S'    
                    {
                        match_count += 1
                    }
                }
            }
        } 
    }

    match_count
}

fn solve_part_2(input: &str) -> u32 {
    let grid = parse_grid(input);

    let corner = GridVec::from([1, 1]);
    let corners = [
        corner,
        corner.rot_90(),
        corner.rot_90().rot_90(),
        corner.rot_90().rot_90().rot_90()
    ];
    let corner_pairs = corners.into_iter()
        .map(|c| [c, c.rot_90()])
        .collect::<Vec<_>>();

    let mut match_count = 0;
    for i in 0..grid.row_count {
        for j in 0..grid.col_count {
            let offset = GridVec::from([i, j]);
            if grid[offset] == 'A' {
                for &[corner1, corner2] in corner_pairs.iter() {
                    if let Some(m1) = grid.get(offset + corner1) && *m1 == 'M'
                        && let Some(m2) = grid.get(offset + corner2) && *m2 == 'M'
                        && let Some(s1) = grid.get(offset - corner1) && *s1 == 'S'
                        && let Some(s2) = grid.get(offset - corner2) && *s2 == 'S'
                    {
                        match_count += 1;
                    }
                }
            }
        } 
    }

    match_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");
    
    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 18);     
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 9);     
    }
}
