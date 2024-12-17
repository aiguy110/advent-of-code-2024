use std::collections::{BTreeMap, BTreeSet};
use grid::GridVec;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BRIGHT_BG: &str = "\x1b[1;47m\x1b[1;30m";
const ANSI_DIM_WHITE: &str = "\x1b[2;37m";


#[derive(Default)]
struct TransmitterMap {
    row_count: usize,
    col_count: usize,
    tx_type_locs: BTreeMap<char, Vec<GridVec>>
} 

impl TransmitterMap {
    fn is_in_borders(&self, loc: GridVec) -> bool {
        loc.i >= 0
        && loc.j >= 0
        && loc.i < self.col_count as i64
        && loc.j < self.col_count as i64
    }

    fn render(&self, antinode_locs: &BTreeSet<GridVec>) {
        let mut char_lookup = BTreeMap::new();
        for (c, locs) in self.tx_type_locs.iter() {
            for loc in locs.iter() {
                char_lookup.insert(*loc, *c);
            }
        }

        for i in 0..self.row_count {
            for j in 0..self.col_count {
                let loc = GridVec::from([i, j]);
                let mut char_to_print = '.';
                if antinode_locs.contains(&loc) {
                    print!("{}", ANSI_BRIGHT_BG); 
                    char_to_print = '#';
                } else {
                    print!("{}", ANSI_DIM_WHITE); 
                }

                if let Some(c) = char_lookup.get(&loc) {
                    char_to_print = *c;
                }

                print!("{}{}", char_to_print, ANSI_RESET);
            }
            println!("");
        }
    }
}

fn main() {
    println!("Answer for Part 1: {}", solve_part_1(PUZZLE_INPUT));
    println!("Answer for Part 2: {}", solve_part_2(PUZZLE_INPUT));
}

fn parse_input(input: &str) -> TransmitterMap {
    let mut tx_map = TransmitterMap::default();

    for (i, line) in input.lines().enumerate() {
        tx_map.row_count = tx_map.row_count.max(i+1);
        for (j, c) in line.trim().chars().enumerate() {
            tx_map.col_count = tx_map.col_count.max(j+1);

            if c == '.' {
                continue
            }

            let loc = GridVec::from([i, j]);
            if let Some(txs) = tx_map.tx_type_locs.get_mut(&c) {
                txs.push(loc);
            } else {
                tx_map.tx_type_locs.insert(c, vec![loc]);
            }
        }
    } 

    tx_map
}

fn solve_part_1(input: &str) -> usize {
    let tx_map = parse_input(input);

    let mut antinode_locs = BTreeSet::<GridVec>::new();
    for (_tx_type, tx_locs) in tx_map.tx_type_locs.iter() {
        for n in 0..tx_locs.len() - 1 {
            for m in n+1..tx_locs.len() {
                let delta = tx_locs[m] - tx_locs[n];

                let antinode_loc = tx_locs[m] + delta;
                if tx_map.is_in_borders(antinode_loc) {
                    antinode_locs.insert(antinode_loc); 
                }
                
                let antinode_loc = tx_locs[n] - delta;
                if tx_map.is_in_borders(antinode_loc) {
                    antinode_locs.insert(antinode_loc); 
                }
            }
        }
    }

    tx_map.render(&antinode_locs);

    antinode_locs.len()
}

fn solve_part_2(input: &str) -> usize {
    let tx_map = parse_input(input);

    let mut antinode_locs = BTreeSet::<GridVec>::new();
    for (_tx_type, tx_locs) in tx_map.tx_type_locs.iter() {
        for n in 0..tx_locs.len() - 1 {
            for m in n+1..tx_locs.len() {
                let delta = tx_locs[m] - tx_locs[n];

                let mut antinode_loc = tx_locs[m];
                while tx_map.is_in_borders(antinode_loc) {
                    antinode_locs.insert(antinode_loc); 
                    antinode_loc += delta;
                }
                
                let mut antinode_loc = tx_locs[n];
                while tx_map.is_in_borders(antinode_loc) {
                    antinode_locs.insert(antinode_loc); 
                    antinode_loc -= delta;
                }
            }
        }
    }

    tx_map.render(&antinode_locs);

    antinode_locs.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 14);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 34);
    }
}


