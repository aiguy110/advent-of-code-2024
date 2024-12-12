use std::collections::BTreeMap;
use grid::{Grid, GridVec};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

struct RegEntry {
    region_id: u32,
    boarder_contrib: u32
}

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> u32 {
    let grid = parse_input(input);

    let mut registry = BTreeMap::new();
    let mut current_region_id = 0;
    for i in 0..grid.row_count {
        for j in 0..grid.col_count {
            let loc = GridVec::from([i, j]);
            if !registry.contains_key(&loc) {
                flood_fill(loc, &grid, current_region_id, &mut registry);
                current_region_id += 1;
            }
        }
    }

    let mut region_totals: BTreeMap<u32, [u32; 2]> = BTreeMap::new();
    for RegEntry { region_id, boarder_contrib } in registry.into_values() {
        if let Some([area, perimeter]) = region_totals.get_mut(&region_id) {
            *area += 1;
            *perimeter += boarder_contrib;
        } else {
            region_totals.insert(region_id, [1, boarder_contrib]);
        }
    }

    region_totals.into_values()
        .map(|[area, perimeter]| area * perimeter)
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    69
}

fn parse_input(input: &str) -> Grid<char> {
    Grid::from_iter(
        input.lines().map(|l| l.trim().chars())
    ).unwrap()

}

fn flood_fill(start: GridVec, grid: &Grid<char>, region_id: u32, registry: &mut BTreeMap<GridVec, RegEntry>) {
    if registry.contains_key(&start) {
        return;
    }

    let mut adj_dirs = vec![GridVec::from([1, 0])];
    for _ in 0..3 {
        adj_dirs.push(adj_dirs.last().unwrap().rot_90());
    }

    let same_color_adj_locs = adj_dirs.into_iter()
        .map(|dir| start + dir)
        .filter(|&loc| match grid.get(loc) { 
            Some(&c) => c == grid[start],
            None => false
        })
        .collect::<Vec<_>>();

    let boarder_contrib = 4 - same_color_adj_locs.len() as u32;
    registry.insert(start, RegEntry { region_id, boarder_contrib });

    let unregged_same_color_adj_locs = same_color_adj_locs.into_iter()
        .filter(|loc| !registry.contains_key(loc))
        .collect::<Vec<_>>();
    
    for loc in unregged_same_color_adj_locs {
        flood_fill(loc, grid, region_id, registry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("../test_input_1.txt");
    const TEST_INPUT_2: &str = include_str!("../test_input_2.txt");
    const TEST_INPUT_3: &str = include_str!("../test_input_3.txt");

    #[test]
    fn test_solve_part_1_test_1() {
        assert_eq!(solve_part_1(TEST_INPUT_1), 140);
    }

    #[test]
    fn test_solve_part_1_test_2() {
        assert_eq!(solve_part_1(TEST_INPUT_2), 772);
    }

    #[test]
    fn test_solve_part_1_test_3() {
        assert_eq!(solve_part_1(TEST_INPUT_3), 1930);
    }
}
