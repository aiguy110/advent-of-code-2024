const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

use std::collections::HashSet;
use grid::GridVec;

#[derive(Default)]
struct State {
    obsticle_locs: HashSet<GridVec>,
    visited_locs: HashSet<GridVec>,
    guard_loc: GridVec,
    guard_dir: GridVec,
    row_count: usize,
    col_count: usize
}

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT))
}

fn solve_part_1(input: &str) -> usize {
    let mut state = parse_state(input);
    while tick(&mut state) {};
    state.visited_locs.len()
}

fn solve_part_2(input: &str) -> usize {
    let mut state = parse_state(input);
}


fn parse_state(input: &str) -> State {
    let mut state = State::default();

    for (i, mut line) in input.lines().enumerate() {
        line = line.trim();
        state.col_count = line.len();
        state.row_count = i+1;
        for (j, c) in line.trim().chars().enumerate() {
            if c == '#' {
                state.obsticle_locs.insert(GridVec::from([i, j]));
            } else if c == '^' {
                state.guard_loc = GridVec::from([i, j]);
            }
        }
    }

    state.guard_dir = GridVec::from([-1, 0]);

    state
}

fn tick(state: &mut State) -> bool {
    let mut next_loc = state.guard_loc + state.guard_dir;

    if next_loc.i < 0 
        || next_loc.j < 0 
        || next_loc.i >= state.row_count as i32
        || next_loc.j >= state.col_count as i32
    {
        return false;
    }

    if state.obsticle_locs.contains(&next_loc) {
        state.guard_dir = state.guard_dir.rot_90_anti();
        next_loc = state.guard_loc + state.guard_dir;
    }
    
    state.visited_locs.insert(next_loc);
    state.guard_loc = next_loc;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(&TEST_INPUT), 41);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(&TEST_INPUT), 6);
    }
}
