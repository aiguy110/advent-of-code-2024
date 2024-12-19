use grid::{GridVec, Grid};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

enum ParseMode {
    Normal,
    Wide
}
use ParseMode::*;

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let (mut grid, moves) = parse_input(input, Normal);
    let mut bot_loc = find_bot(&grid);

    for dir in moves {
        println!("{:?}", dir);
        grid.render();
        println!();
        bot_loc = do_move(&mut grid, bot_loc, dir);
    }

    grid.rows.into_iter()
        .enumerate()
        .map(|(i, row)| row.into_iter()
             .enumerate()
             .filter_map(move |(j, c)| if c == 'O' {Some(100*i + j)} else { None })
        )
        .flatten()
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let (mut grid, moves) = parse_input(input, Wide);
    let mut bot_loc = find_bot(&grid);

    for dir in moves {
        bot_loc = do_move(&mut grid, bot_loc, dir);
    }

    grid.rows.into_iter()
        .enumerate()
        .map(|(i, row)| row.into_iter()
             .enumerate()
             .filter_map(move |(j, c)| if c == '[' {Some(100*i + j)} else { None })
        )
        .flatten()
        .sum()
}

fn parse_input(input: &str, mode: ParseMode) -> (Grid<char>, Vec<GridVec>) {
    let mut lines = input.lines().map(|l| l.trim());

    // First pull out the lines about the grid. We'll process these later.
    let mut grid_lines = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }

        grid_lines.push(line);
    }

    // Remaining lines will be about moves
    let moves = lines.map(|l| l.chars())
        .flatten()
        .map(|c| match c {
            '^' => GridVec::from([-1, 0]),
            '>' => GridVec::from([ 0, 1]),
            'v' => GridVec::from([ 1, 0]),
            '<' => GridVec::from([ 0,-1]),
            _ => panic!("Invalid direction character '{}'", c)
        })
        .collect();

    (
        match mode {
            Normal => Grid::from_iter(grid_lines.into_iter().map(|l| l.chars())).unwrap(),
            Wide => Grid::from_iter(grid_lines.into_iter()
                .map(|l| l.chars()
                     .map(|c| match c {
                         '.' => "..".chars(),
                         'O' => "[]".chars(),
                         '#' => "##".chars(),
                         '@' => "@.".chars(),
                         _ => panic!("Invalid grid character '{}'", c)
                     })
                     .flatten()
            )).unwrap()
        },
        moves
    )
}

/// Bot steps in `bot_dir` from `bot_loc` (if possible) pushing all boxes in its path.
/// Function returns new `bot_loc`, regardless of whether or not it changed.
fn do_move(grid: &mut Grid<char>, bot_loc: GridVec, bot_dir: GridVec) -> GridVec {
    match locs_to_slide(grid, &vec![bot_loc], bot_dir) {
        None => return bot_loc,
        Some(slide_locs) => {
            println!("{:?}", slide_locs);
            for loc in slide_locs {
                grid[loc+bot_dir] = grid[loc];
                grid[loc] = '.';
            }
        }
    }

    bot_loc + bot_dir
}

/// Finds the location the first '@' on the grid or panics
fn find_bot(grid: &Grid<char>) -> GridVec {
    for (i, row) in grid.rows.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                return GridVec::from([i, j]);
            }
        }
    }

    panic!("Didn't find '@' in grid");
}

/// Recursively determines all locations which must be slid in direction `dir` to accomdate sliding
/// `locs`, and returns a `Some(vec![...])` of these locations with more distant locations at the front
/// of the Vec. If an obstuction is encounterd, returns `None`
fn locs_to_slide(grid: &Grid<char>, locs: &Vec<GridVec>, dir: GridVec) -> Option<Vec<GridVec>> {
    if locs.len() == 0 {
        return Some(vec![]);
    }
    
    let mut new_slide_locs = vec![];
    for &loc in locs.iter() {
        match grid[loc] {
            '#' => return None,
            'O' | '@' => new_slide_locs.extend(
                match locs_to_slide(grid, &vec![loc + dir], dir) {
                    Some(v) => v,
                    None => return None
                }),
            '[' => match dir {
                GridVec {i: 0, j: 1} => {
                    new_slide_locs.push(loc + dir);
                    new_slide_locs.extend(match locs_to_slide(grid, &vec![loc + 2*dir], dir) { Some(v) => v, None => return None})
                },
                GridVec {i: 0, j:-1} => {
                    new_slide_locs.extend(match locs_to_slide(grid, &vec![loc + dir], dir) { Some(v) => v, None => return None})
                },
                _ /* vertical */ => { 
                    new_slide_locs.extend(match locs_to_slide(grid, &vec![loc + dir, loc + dir + GridVec::from([0, 1])], dir) { Some(v) => v, None => return None}) 
                }
            },
            ']' => match dir {
                GridVec {i: 0, j: 1} => {
                    new_slide_locs.extend(match locs_to_slide(grid, &vec![loc + dir], dir) { Some(v) => v, None => return None})
                },
                GridVec {i: 0, j:-1} => {
                    new_slide_locs.push(loc + dir);
                    new_slide_locs.extend(match locs_to_slide(grid, &vec![loc + 2*dir], dir) { Some(v) => v, None => return None} )
                },
                _ /* vertical */ => { 
                    new_slide_locs.extend(match locs_to_slide(grid, &vec![ loc + dir, loc + dir + GridVec::from([0,-1]) ], dir) { Some(v) => v, None => return None})
                }
            },
            _ => {}
        }
    }

    new_slide_locs.extend_from_slice(&locs[..]);
    Some(new_slide_locs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 10092);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 42);
    }
}
