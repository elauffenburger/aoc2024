use std::{
    error, fs,
    io::{self},
};

use crate::{guard_map, Cli};

pub fn part_one(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let mut map = guard_map::Map::from_reader(file_reader)?;

    // Let the map tick until it's done.
    while map.tick()? {
        if args.debug {
            println!("{:?}", map);
        }
    }

    // Check the number of unique positions the guard went to.
    println!("{}", map.guard_visited_positions().collect::<Vec<_>>().len());

    Ok(())
}
