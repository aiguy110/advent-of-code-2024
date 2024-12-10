use std::collections::BTreeMap;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    println!("Part 1 Answer: {}", solve_part_1(PUZZLE_INPUT));
    println!("Part 2 Answer: {}", solve_part_2(PUZZLE_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    let mut mem = parse_input(input);
    compress_mem(&mut mem);
    calc_mem_hash(&mem)
}

fn solve_part_2(input: &str) -> usize {
    let mut mem = parse_input(input);
    compress_mem_2(&mut mem);
    calc_mem_hash(&mem)
}

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut mem = Vec::new();

    let mut id = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..n {
                mem.push(Some(id));
            }
            id += 1
        }  else {
            for _ in 0..n {
                mem.push(None);
            }
        }
    }

    mem
}

fn compress_mem(mem: &mut Vec<Option<u32>>) {
    let mut first_free_ind = 0;
    let mut last_full_ind = mem.len()-1;

    loop {
        while first_free_ind < last_full_ind && matches!(mem[first_free_ind], Some(_)) {
            // 2nd condition above mean first_free_ind is not in fact free, so seek right
            first_free_ind += 1;
        }
        while first_free_ind < last_full_ind && matches!(mem[last_full_ind], None) {
            // 2nd condition above mean last_full_ind is not in fact full, so seek left
            last_full_ind -= 1;
        }

        if first_free_ind < last_full_ind {
            // We're not done yet, so move the full mem cell into the free mem cell
            mem.swap(last_full_ind, first_free_ind);
        } else {
            // We're done
            break
        }
    }
}

// This is deffinitely not the most efficient way to do this, but I am lazy and my CPU is not.
fn compress_mem_2(mem: &mut Vec<Option<u32>>) {
    let max_id = mem.iter()
        .max_by_key(|cell| if let Some(n) = cell {*n} else {0})
        .unwrap()
        .unwrap() + 1;

    for id in (0..max_id).rev()
    {
        let (blocks, voids) = get_blocks_and_voids(&mem);
        let &[block_start, block_len] = blocks.get(&id).unwrap();
        for &[void_start, void_len] in voids.iter() {
            if void_len >= block_len && void_start < block_start {
                for i in 0..block_len {
                    mem.swap(block_start+i, void_start+i);
                }
            }
        }
    }
}

fn get_blocks_and_voids(mem: &Vec<Option<u32>>) -> (BTreeMap<u32, [usize; 2]>, Vec<[usize; 2]>) {
    let mut blocks = BTreeMap::new();
    let mut in_block = false;
    let mut block_start = 0;
    let mut block_id = 0;

    let mut voids = Vec::new();
    let mut in_void = false;
    let mut void_start = 0;

    for i in 0..mem.len() {
        if let Some(id) = mem[i] {
            if !in_block {
                in_block = true;
                block_start = i;
                block_id = id;
            } else if block_id != id {
                blocks.insert(block_id, [block_start, i-block_start]);
                block_id = id;
                block_start = i;
            }
            if in_void {
                in_void = false;
                voids.push([void_start, i-void_start])
            }
        } else {
            if !in_void {
                in_void = true;
                void_start = i;
            }
            if in_block {
                in_block = false;
                blocks.insert(block_id, [block_start, i-block_start]);
            }
        }
    }

    if in_block {
        blocks.insert(block_id, [block_start, mem.len()-block_start]);
    }

    (blocks, voids)
}

fn calc_mem_hash(mem: &[Option<u32>]) -> usize {
    mem.iter()
        .enumerate()
        .filter_map(|(i, &id)| id.map(|n| i * n as usize))
        .sum()
}

fn render_mem(mem: &Vec<Option<u32>>) -> String {
    let mut s = String::new();
    for cell in mem.iter() {
        match cell {
            Some(id) => s = format!("{}{}", s, id),
            None => s = format!("{}.", s)
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(TEST_INPUT), 1928);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(TEST_INPUT), 2858);
    }
}
