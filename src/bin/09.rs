/*

https://adventofcode.com/2024/day/9

part 1:

might be an awkward way to iterate intervals, but we consider each index in turn, and if it's empty then pull from the end of the file.

part 2:

In this one the blocks stay contiguous, so I thought of them more as actual intervals. Went linked list style to track which blocks are
next to which. Might have been overkill: shifting the Vec around would probably have been fine. Just need to track both the current
position of each block (indexed by block id), and also the Vec of blocks in current order.

This solution is incorrect for the case when you're considering moving a block to just after the block that already preceeds it
We know that the block will always fit in this case, but this code only accepts the move if the gap between the two blocks is large enough
(Even though the block is moving so there will be space).
Submitted and it was correct, so I guess the input doesn't have this case.

*/

fn main() {
    let input = parse(include_str!("../../input/09.txt"));

    let part1 = part1(&input);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 6337921897505);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 6362722604045);
}

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn part1(input: &Input) -> i64 {
    // Start by calculating [start, end) for each block
    let mut blocks = Vec::<(i64, i64)>::new();
    let mut i = 0;
    for (idx, c) in input.iter().enumerate() {
        if idx % 2 == 0 {
            // file block
            // let file_id = idx / 2;
            let start = i;
            let end = i + c;
            blocks.push((start, end));
            i += c;
        } else {
            // space block
            i += c;
        }
    }

    // Now iterate forwards, and if we have a gap, pull from the end
    let mut next_block_id = 0;
    let mut i = 0;
    let mut checksum = 0;
    loop {
        if next_block_id >= blocks.len() {
            break;
        }
        let block = &blocks[next_block_id];
        if i < block.0 {
            // We have a gap at position i
            let final_block_idx = blocks.len() - 1;
            // println!("i:{} = Gap (Block = {})", i, final_block_idx);

            checksum += i * final_block_idx as i64;

            // Shorten the block by 1
            blocks[final_block_idx].1 -= 1;
            if blocks[final_block_idx].0 >= blocks[final_block_idx].1 {
                // Block is empty, remove it
                blocks.pop();
            }

            i += 1;
        } else {
            if i < block.1 {
                // We are in the block
                // println!("i:{} = Block {}", i, next_block_id);
                checksum += i * next_block_id as i64;
                i += 1;
            } else {
                // We are at the end of the block
                next_block_id += 1;
            }
        }
    }

    checksum
}

struct Block {
    start: i64,
    end: i64,
    prev_block_id: Option<usize>,
    next_block_id: Option<usize>,
}

fn part2(input: &Input) -> i64 {
    // Start by calculating [start, end) for each block
    let mut blocks = Vec::<Block>::new();
    let mut i = 0;
    for (idx, c) in input.iter().enumerate() {
        if idx % 2 == 0 {
            // file block
            let start = i;
            let end = i + c;
            blocks.push(Block {
                start,
                end,
                next_block_id: None,
                prev_block_id: None,
            });
            i += c;
        } else {
            // space block
            i += c;
        }
    }
    for block_idx in 0..blocks.len() {
        if block_idx > 0 {
            blocks[block_idx].prev_block_id = Some(block_idx - 1);
        }
        if block_idx < blocks.len() - 1 {
            blocks[block_idx].next_block_id = Some(block_idx + 1);
        }
    }

    // Attempt to move each block exactly once
    for block_move_idx in (0..blocks.len()).rev() {
        // println!("Block Move Candidate: {}", block_move_idx);
        let block_sz = blocks[block_move_idx].end - blocks[block_move_idx].start;

        // Consider moving to directly after block_idx
        let mut block_idx = 0;
        loop {
            if block_idx == block_move_idx {
                break;
            }
            let next_block_idx = blocks[block_idx].next_block_id.unwrap();
            // @@ need to consider seperately next_block_idx == block_move_idx
            let gap_after = blocks[next_block_idx].start - blocks[block_idx].end;
            if gap_after >= block_sz {
                // println!("Move {} to after {}", block_move_idx, block_idx);
                // Move block_move_idx to block_idx
                if let Some(prev_block_id) = blocks[block_move_idx].prev_block_id {
                    blocks[prev_block_id].next_block_id = blocks[block_move_idx].next_block_id;
                }
                blocks[block_move_idx].start = blocks[block_idx].end;
                blocks[block_move_idx].end = blocks[block_idx].end + block_sz;
                blocks[block_move_idx].next_block_id = blocks[block_idx].next_block_id;
                blocks[block_move_idx].prev_block_id = Some(block_idx);

                blocks[block_idx].next_block_id = Some(block_move_idx);
                break;
            }
            block_idx = next_block_idx;
        }
    }

    let mut checksum = 0;
    for (idx, block) in blocks.iter().enumerate() {
        for k in block.start..block.end {
            // println!("{}: {}", k, idx);
            checksum += idx as i64 * k;
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse(include_str!("../../input/09_example.txt"));
        assert_eq!(part1(&input), 1928);
        assert_eq!(part2(&input), 2858);
    }
}
