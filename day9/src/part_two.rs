use std::{error, fs, io};

use crate::{disk_map, Cli};

pub fn part_two(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let disk_map::NewDiskMap {
        mut disk_map,
        mut blocks,
        mut free_space_regions,
    } = disk_map::DiskMap::from_reader(file_reader)?;

    // for block in blocks.rev():
    for i in (0..blocks.len()).rev() {
        if args.debug {
            let block = &blocks[i];

            println!("{}", &disk_map);
            dbg!(&blocks, &block, &free_space_regions);
        }

        let block = &mut blocks[i];

        // region <- first free region with enough space
        let region_i = 'block: {
            for (i, region) in free_space_regions.iter().enumerate() {
                if region.start < block.start && region.size >= block.size {
                    break 'block Some(i);
                }
            }

            None
        };

        match region_i {
            // if region is none, go to the next block
            None => continue,

            // otherwise:
            Some(region_i) => {
                let region = &mut free_space_regions[region_i];
                let region_orig_start = region.start;

                // move the block to the start of the region
                for i in region.start..(region.start + block.size as usize) {
                    for j in block.start..(block.start + block.size as usize) {
                        disk_map.disk.swap(i, j);
                    }
                }

                // update the region metadata:
                region.start += block.size as usize;
                region.size -= block.size;

                // if the region is now empty, delete it!
                if region.size == 0 {
                    free_space_regions.remove(region_i);
                }

                // update the block metadata (even though this doesn't really matter).
                block.start = region_orig_start;
            }
        }
    }

    if args.debug {
        println!("{}", &disk_map);
    }

    // HACK: we could totally calculate this on the fly, but let's just do it after the fact for now.
    let checksum = {
        let mut checksum = 0u64;
        for (i, entry) in disk_map.disk.iter().enumerate() {
            match entry {
                disk_map::DiskMapEntry::FileBlock(id) => checksum += *id as u64 * i as u64,
                disk_map::DiskMapEntry::FreeSpace => continue,
            }
        }

        checksum
    };

    println!("{}", checksum);

    Ok(())
}
