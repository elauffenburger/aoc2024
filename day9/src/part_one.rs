use std::{
    error, fs,
    io::{self},
};

use crate::disk_map;

use crate::Cli;

pub fn part_one(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let disk_map::NewDiskMap {
        mut disk_map,
        blocks,
        free_space_regions,
    } = disk_map::DiskMap::from_reader(file_reader)?;

    // start left at the first free space and right at the last block.
    let mut left = free_space_regions[0].start;
    let mut right = {
        let block = blocks.last().unwrap();

        block.start + block.size as usize - 1
    };

    // while left < right:
    while left < right {
        //   swap left and right
        disk_map.disk.swap(left, right);

        if args.debug {
            println!("{}", &disk_map);
            println!("{}l{}r", " ".repeat(left), " ".repeat(right - left - 1));
        }

        // move left -> and move right <-
        left += 1;
        right -= 1;

        // while left is not a free block and left < right, move left ->
        while left < right {
            match disk_map.disk[left] {
                disk_map::DiskMapEntry::FreeSpace => break,
                disk_map::DiskMapEntry::FileBlock(_) => {
                    left += 1;
                }
            }
        }

        // while right is not a block and right > left, move right <-
        while right > left {
            match disk_map.disk[right] {
                disk_map::DiskMapEntry::FreeSpace => right -= 1,
                disk_map::DiskMapEntry::FileBlock(_) => break,
            }
        }
    }

    // HACK: we could totally calculate this on the fly, but let's just do it after the fact for now.
    let checksum = {
        let mut checksum = 0u64;
        for (i, entry) in disk_map.disk.iter().enumerate() {
            match entry {
                disk_map::DiskMapEntry::FileBlock(id) => checksum += *id as u64 * i as u64,
                disk_map::DiskMapEntry::FreeSpace => break,
            }
        }

        checksum
    };

    println!("{}", checksum);

    Ok(())
}
