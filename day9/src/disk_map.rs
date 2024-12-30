use core::fmt;
use std::{error, fmt::Write, io::BufRead};

#[derive(Debug)]
pub enum DiskMapEntry {
    FileBlock(u32),
    FreeSpace,
}

#[derive(Debug)]
pub struct DiskMap {
    pub disk: Vec<DiskMapEntry>,
}

#[derive(Debug)]
pub struct NewDiskMap {
    pub disk_map: DiskMap,

    pub blocks: Vec<BlockInfo>,
    pub free_space_regions: Vec<FreeSpaceRegion>,
}

#[derive(Debug)]
pub struct BlockInfo {
    pub start: usize,
    pub size: u32,
}

#[derive(Debug)]
pub struct FreeSpaceRegion {
    pub start: usize,
    pub size: u32,
}

impl DiskMap {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<NewDiskMap, Box<dyn error::Error>> {
        let mut disk = vec![];
        let mut blocks = vec![];
        let mut free_space_regions = vec![];

        let line = reader.lines().next().ok_or("expected input")??;

        let mut i = 0usize;
        let mut in_free_space = false;
        let mut id = 0u32;

        for ch in line.chars() {
            let num: u32 = ch
                .to_digit(10)
                .expect("expected character to be a valid u32");

            if num != 0 {
                if in_free_space {
                    free_space_regions.push(FreeSpaceRegion {
                        start: i,
                        size: num,
                    });

                    let n = i + num as usize;
                    while i < n {
                        disk.push(DiskMapEntry::FreeSpace);
                        i += 1
                    }
                } else {
                    blocks.push(BlockInfo {
                        start: i,
                        size: num,
                    });

                    let n = i + num as usize;
                    while i < n {
                        disk.push(DiskMapEntry::FileBlock(id));
                        i += 1
                    }

                    id += 1;
                }
            }

            in_free_space = !in_free_space;
        }

        Ok(NewDiskMap {
            disk_map: DiskMap { disk },
            blocks,
            free_space_regions,
        })
    }
}

impl fmt::Display for DiskMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.disk.iter() {
            match entry {
                DiskMapEntry::FileBlock(id) => f.write_fmt(format_args!("{}", id))?,
                DiskMapEntry::FreeSpace => f.write_char('.')?,
            }
        }

        Ok(())
    }
}
