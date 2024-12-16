use std::collections::BTreeMap;
use grid::{Grid, GridVec};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BRIGHT_BG: &str = "\x1b[1;47m\x1b[1;30m";
const ANSI_BG_RED: &str = "\x1b[41m";
const ANSI_BG_GREEN: &str = "\x1b[42m";
const ANSI_BG_BLUE: &str = "\x1b[44m";

struct RegEntry {
    region_id: u32,
    boarder_contrib: u32,
    corner_contrib: u32
}

enum Part {
    One,
    Two
}

fn main() {
    println!("Part 1 Answer: {}", solve(PUZZLE_INPUT, Part::One));
    println!("Part 2 Answer: {}", solve(PUZZLE_INPUT, Part::Two));
}

fn solve(input: &str, part: Part) -> u32 {
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

    let mut region_totals: BTreeMap<u32, [u32; 3]> = BTreeMap::new();
    let mut corner_locs = BTreeMap::new();
    for (loc, RegEntry { region_id, boarder_contrib, corner_contrib }) in registry.into_iter() {
        if let Some([area, perimeter, corners]) = region_totals.get_mut(&region_id) {
            *area += 1;
            *perimeter += boarder_contrib;
            *corners += corner_contrib;
        } else {
            region_totals.insert(region_id, [1, boarder_contrib, corner_contrib]);
        }

        if corner_contrib > 0 {
            corner_locs.insert(loc, corner_contrib);
        }
    }

    render_with_highlight_levels(&grid, &corner_locs);

    match part {
        Part::One => {
            region_totals.into_values()
                .map(|[area, perimeter, _]| area * perimeter)
                .sum()
        },
        Part::Two => {
            region_totals.into_values()
                .map(|[area, _, corners]| area * corners)
                .sum()
        }
    }
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

    let corner_loc_triples = adj_dirs.iter()
        .zip(adj_dirs[1..].iter().chain(adj_dirs[..1].iter()))
        .map(|(&dir1, &dir2)| (start + dir1, start + dir2, start + dir1 + dir2))
        .collect::<Vec<_>>();

    let same_color_adj_locs = adj_dirs.into_iter()
        .map(|dir| start + dir)
        .filter(|&loc| match grid.get(loc) { 
            Some(&c) => c == grid[start],
            None => false
        })
        .collect::<Vec<_>>();

    let corner_contrib = corner_loc_triples.iter()
        .map(|(loc1, loc2, loc3)| {
            !same_color_adj_locs.contains(loc1) && !same_color_adj_locs.contains(loc2) // convex
            || same_color_adj_locs.contains(loc1) && same_color_adj_locs.contains(loc2) && (grid.get(*loc3) != Some(&grid[start]))// concave
        })
        .filter(|is_corner| *is_corner)
        .count() as u32;

    let boarder_contrib = 4 - same_color_adj_locs.len() as u32;
    registry.insert(start, RegEntry { region_id, boarder_contrib, corner_contrib });

    let unregged_same_color_adj_locs = same_color_adj_locs.into_iter()
        .filter(|loc| !registry.contains_key(loc))
        .collect::<Vec<_>>();
    
    for loc in unregged_same_color_adj_locs {
        flood_fill(loc, grid, region_id, registry);
    }
}

fn render_with_highlight_levels(grid: &Grid<char>, highlights: &BTreeMap<GridVec, u32>) {
    for i in 0..grid.row_count {
        for j in 0..grid.col_count {
            let loc = GridVec::from([i, j]);
            let level = highlights.get(&loc);
            match level {
                Some(1) => print!("{}{}", ANSI_BRIGHT_BG, ANSI_BG_RED),
                Some(2) => print!("{}{}", ANSI_BRIGHT_BG, ANSI_BG_GREEN),
                Some(3) => print!("{}{}", ANSI_BRIGHT_BG, ANSI_BG_BLUE),
                Some(4) => print!("{}", ANSI_BRIGHT_BG),
                _ => {}
            }
            print!("{}", grid[loc]);
            print!("{}", ANSI_RESET);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("../test_input_1.txt");
    const TEST_INPUT_2: &str = include_str!("../test_input_2.txt");
    const TEST_INPUT_3: &str = include_str!("../test_input_3.txt");
    const TEST_INPUT_4: &str = include_str!("../test_input_4.txt");
    const TEST_INPUT_5: &str = include_str!("../test_input_5.txt");

    #[test]
    fn test_solve_part_1_test_1() {
        assert_eq!(solve(TEST_INPUT_1, Part::One), 140);
    }

    #[test]
    fn test_solve_part_1_test_2() {
        assert_eq!(solve(TEST_INPUT_2, Part::One), 772);
    }

    #[test]
    fn test_solve_part_1_test_3() {
        assert_eq!(solve(TEST_INPUT_3, Part::One), 1930);
    }

    #[test]
    fn test_solve_part_2_test_1() {
        assert_eq!(solve(TEST_INPUT_1, Part::Two), 80);
    }

    #[test]
    fn test_solve_part_2_test_2() {
        assert_eq!(solve(TEST_INPUT_2, Part::Two), 436);
    }

    #[test]
    fn test_solve_part_2_test_3() {
        assert_eq!(solve(TEST_INPUT_3, Part::Two), 1206);
    }

    #[test]
    fn test_solve_part_2_test_4() {
        assert_eq!(solve(TEST_INPUT_4, Part::Two), 236);
    }

    #[test]
    fn test_solve_part_2_test_5() {
        assert_eq!(solve(TEST_INPUT_5, Part::Two), 368);
    }
}
