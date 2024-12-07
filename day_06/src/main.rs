const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

use std::collections::{HashSet, HashMap};
use grid::GridVec;

#[derive(Default)]
struct State {
    obsticle_locs: HashSet<GridVec>,
    visited_loc_dirs: HashMap<GridVec, Vec<GridVec>>,
    guard_loc: GridVec,
    guard_dir: GridVec,
    row_count: usize,
    col_count: usize
}

enum TickOutcome {
    Running,
    Finished,
    LoopDetected,
}

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let mut state = parse_state(input);
    while matches!(tick(&mut state), TickOutcome::Running) {};
    state.visited_loc_dirs.len()
}

fn solve_part_2(input: &str) -> usize {
    let mut orig_state = parse_state(input);
    while matches!(tick(&mut orig_state), TickOutcome::Running) {};

    let mut vialble_locs = 0;
    for candidate_loc in orig_state.visited_loc_dirs.into_keys() {
        let mut sim_state = parse_state(input);
        sim_state.obsticle_locs.insert(candidate_loc);

        let mut continue_sim = true;
        while continue_sim {
            let outcome = tick(&mut sim_state);
            if !matches!(outcome, TickOutcome::Running) {
                continue_sim = false;
            }
            if matches!(outcome, TickOutcome::LoopDetected) {
                vialble_locs += 1;
            }
        };
    }

    vialble_locs
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

fn tick(state: &mut State) -> TickOutcome {
    // Assume we'll go straight
    let mut next_loc = state.guard_loc + state.guard_dir;

    // If going straight takes us off the map, we're done
    if next_loc.i < 0 
        || next_loc.j < 0 
        || next_loc.i >= state.row_count as i32
        || next_loc.j >= state.col_count as i32
    {
        return TickOutcome::Finished;
    }

    // If going straight would put us in an obsticle, we actually need to turn right
    if state.obsticle_locs.contains(&next_loc) {
        state.guard_dir = state.guard_dir.rot_90_anti();
        next_loc = state.guard_loc + state.guard_dir;
    }

    // At this point we know where we're going next. Check if we've been there before
    if let Some(dirs) = state.visited_loc_dirs.get_mut(&next_loc) {
        // ... we've been here. Have we been here walking in the same direction?
        if dirs.contains(&state.guard_dir) {
            return TickOutcome::LoopDetected;
        } else {
            dirs.push(state.guard_dir)
        }
    } else {
        // ... we've never been here before
        state.visited_loc_dirs.insert(next_loc, vec![state.guard_dir]);
    }
    
    state.guard_loc = next_loc;
    TickOutcome::Running
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
