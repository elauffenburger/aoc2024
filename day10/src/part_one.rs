use std::{
    collections::HashSet, error, fs, io::{self}
};

use crate::topo;

use crate::Cli;

pub fn part_one(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let map = topo::Map::from_reader(file_reader)?;

    // Find paths using a HashSet so we get paths to unique locations.
    let paths = topo::find_paths::<HashSet<topo::Location>>(args, &map);

    println!("{}", paths.len());

    Ok(())
}
