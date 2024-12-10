use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct File {
    size: usize,
    id: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Space {
    size: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Block {
    File(File),
    Space(Space),
}

#[derive(Debug)]
struct Disk {
    blocks: VecDeque<Block>,
}

impl Disk {
    fn new(disk_map: &str) -> Self {
        let blocks = disk_map
            .chars()
            .enumerate()
            .map(|(id, char)| {
                let size: usize = char.to_digit(10).unwrap() as usize;
                if id % 2 == 0 {
                    Block::File(File { size, id: id / 2 })
                } else {
                    Block::Space(Space { size })
                }
            })
            .collect();
        Disk { blocks }
    }

    fn compact(&mut self) {
        while let Some(first_space_index) = self
            .blocks
            .iter()
            .position(|block| matches!(block, Block::Space { .. }))
        {
            let available_space =
                if let Block::Space(Space { size }) = self.blocks[first_space_index] {
                    size
                } else {
                    0
                };

            // Find the last File block
            if let Some(last_file_index) = self
                .blocks
                .iter()
                .rposition(|block| matches!(block, Block::File { .. }))
            {
                if last_file_index <= first_space_index {
                    // If the last file index is before or at the first space index, no valid moves are possible
                    break;
                }

                if let Block::File(File {
                    size: file_size,
                    id,
                }) = self.blocks[last_file_index]
                {
                    if file_size <= available_space {
                        if let Block::Space(Space { size }) = &mut self.blocks[first_space_index] {
                            *size -= file_size; // Reduce space by file size
                        }
                        let file_block = self.blocks.remove(last_file_index).unwrap();
                        self.blocks.insert(first_space_index, file_block);

                        if first_space_index + 1 < self.blocks.len() {
                            if let Block::Space(Space { size }) = self.blocks[first_space_index + 1]
                            {
                                if size == 0 {
                                    self.blocks.remove(first_space_index + 1);
                                }
                            }
                        }
                    } else {
                        let remaining_size = file_size - available_space;

                        if let Block::File(File { size, id: _id }) =
                            &mut self.blocks[last_file_index]
                        {
                            *size = remaining_size;
                        }

                        let new_file_block = Block::File(File {
                            size: available_space,
                            id,
                        });

                        self.blocks[first_space_index] = new_file_block;

                        if first_space_index + 1 < self.blocks.len() {
                            if let Block::Space(Space { size }) =
                                &mut self.blocks[first_space_index + 1]
                            {
                                if *size == 0 {
                                    self.blocks.remove(first_space_index + 1);
                                }
                            }
                        }
                    }
                }
            } else {
                // No more files to move
                break;
            }
        }
    }

    fn whole_block_reformat(&mut self) {
        let mut moved: HashSet<usize> = HashSet::new();
        let mut map = self.blocks.clone();
        let mut result: VecDeque<Block> = VecDeque::new();

        while let Some(block) = map.pop_back() {
            match block {
                Block::File(File { size: length, id }) => {
                    if !moved.insert(id) {
                        result.push_front(block);
                        continue;
                    }

                    let mut found_idx = 0;
                    let mut free_space_remaining = 0;
                    for (i, lblock) in map.iter().enumerate() {
                        match lblock {
                            Block::Space(Space { size: free_size }) => {
                                if *free_size >= length {
                                    found_idx = i;
                                    free_space_remaining = *free_size - length;
                                    map.push_back(Block::Space(Space { size: length }));
                                    break;
                                }
                            }
                            Block::File(File { .. }) => {}
                        }
                    }

                    if found_idx != 0 {
                        map[found_idx] = block;
                        if free_space_remaining > 0 {
                            map.insert(
                                found_idx + 1,
                                Block::Space(Space {
                                    size: free_space_remaining,
                                }),
                            );
                        }
                    } else {
                        result.push_front(block);
                    }
                }
                Block::Space(Space { .. }) => {
                    result.push_front(block);
                }
            }
        }

        self.blocks = result;
    }

    fn compute_checksum(&self) -> usize {
        let mut current_index = 0; // Initialize a variable to track the current index
        let mut contributions: Vec<usize> = Vec::new(); // Vector to hold contributions

        // Iterate over blocks and collect contributions
        for block in &self.blocks {
            match block {
                Block::File(File { id, size }) => {
                    // For each file block, calculate contributions for each offset
                    for offset in 0..*size {
                        let contribution = (current_index + offset) * id;
                        contributions.push(contribution); // Collect contribution
                    }
                    current_index += size; // Increment index by the size of the file block
                }
                Block::Space(Space { size }) => {
                    // Spaces contribute 0 to the checksum, but we still increment the index
                    current_index += size; // Increment index for the space block
                }
            }
        }

        // Sum contributions in parallel
        contributions.par_iter().sum() // Return the total checksum
    }

    fn _display(&self) -> String {
        self.blocks
            .par_iter()
            .map(|block| match block {
                Block::File(File { size, id }) => id.to_string().repeat(*size),
                Block::Space(Space { size }) => ".".repeat(*size),
            })
            .collect::<String>()
    }
}

fn main() {
    let start_time = Instant::now();

    let input = include_str!("../sample.txt");

    let mut disk_part1 = Disk::new(input);
    let mut disk_part2 = Disk::new(input);

    disk_part1.compact();

    //let display_part1 = disk_part1.display();
    // println!("Part 1 display: {display_part1}");

    let checksum_part1 = disk_part1.compute_checksum();
    println!("Part 1 checksum: {checksum_part1}");

    disk_part2.whole_block_reformat();

    //let display_part2 = disk_part2.display();
    //println!("Part 2 display: {display_part2}");

    let checksum_part2 = disk_part2.compute_checksum();
    println!("Part 2 checksum: {checksum_part2}");

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds, {} milliseconds, and {} nanoseconds to calculate the antinodes",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_nanos() % 1_000_000
    );
}
