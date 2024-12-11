use std::{fs, usize};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");
    let mut disk_layout: Vec<usize> = Vec::new();
    let mut block_layout: Vec<SuperBlock> = Vec::new();
    for (i, c) in contents.trim().chars().enumerate() {
        let blocks: u32 = c.to_digit(10).unwrap();

        if blocks > 0 {
            block_layout.push(
                SuperBlock{
                    size: blocks,
                    content: match i % 2 {
                        0 => Some(i / 2),
                        _ => None
                    }
                }
            );
        }
        for _block in 0..blocks {
            match i % 2 {
                0 => { disk_layout.push(i / 2); },
                _ => { disk_layout.push(usize::MAX); },
            }
        }
    }

    'forward:
    for i in 0..disk_layout.len() {
        match disk_layout[i] {
            usize::MAX => {
                for j in (0..disk_layout.len()).rev() {
                    if j == i {
                        break 'forward;
                    }
                    match disk_layout[j] {
                            usize::MAX => { continue; },
                            _ => {
                                disk_layout.swap(i, j);
                                break;
                            }
                    }
                }
            },
            _ => { continue; },
        }
    }

    let mut acc: usize = 0;
    for (i, id) in disk_layout.iter().enumerate() {
        if *id == usize::MAX {
            break;
        }
        acc += i * id;
    }

    println!("Checksum 1: {acc}");

    let original_layout: Vec<SuperBlock> = block_layout.clone();
    let mut smallest_seen = usize::MAX;
    'superblock:
    for superblock in original_layout.iter().rev() {
        match superblock.content {
            Some(id) => {
                if id >= smallest_seen {
                    continue;
                }
                smallest_seen = id;
                let mut fitting_block_index: usize = usize::MAX;
                for (i, candidate) in block_layout.iter().enumerate() {
                    if candidate == superblock {
                        continue 'superblock;
                    }
                    if candidate.content != None {
                        continue;
                    }
                    if candidate.size >= superblock.size {
                        fitting_block_index = i;
                        break;
                    }
                }
                if fitting_block_index == usize::MAX {
                    continue;
                }
                let fitting_block_size = block_layout[fitting_block_index].size;
                
                for (index, sb) in block_layout.iter().enumerate() {
                    if sb == superblock { 
                        block_layout.swap(index, fitting_block_index);
                        if fitting_block_size != superblock.size {
                            block_layout[index].size = superblock.size;
                        }
                        break;
                    }
                }
                if fitting_block_size != superblock.size {
                    block_layout.insert(
                        fitting_block_index + 1,
                        SuperBlock{
                            size: fitting_block_size - superblock.size,
                            content: None
                        }
                    );
                }
            },
            None => { continue; }
        };
    }

    let mut disk_layout2: Vec<usize> = Vec::new();
    for block in block_layout {
        disk_layout2.append(&mut block.to_disk_layout());
    }

    let mut acc: usize = 0;
    for (i, id) in disk_layout2.iter().enumerate() {
        if *id == usize::MAX {
            continue;
        }
        acc += i * id;
    }

    println!("Checksum 2: {acc}");

}

#[derive(Clone, Debug, PartialEq)]
struct SuperBlock {
    size: u32,
    content: Option<usize>,
}

impl SuperBlock {
    fn to_disk_layout(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for _ in 0..self.size {
            result.push(match self.content {
                Some(id) => id,
                None => usize::MAX
            });
        }
        result
    }
}
