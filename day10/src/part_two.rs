use std::{
    error, fs,
    io::{self},
};

use crate::topo;

use crate::Cli;

pub fn part_two(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let map = topo::Map::from_reader(file_reader)?;

    // Find paths using a Vec so we get all paths.
    let paths = topo::find_paths::<Vec<topo::Location>>(args, &map);

    println!("{}", paths.len());

    Ok(())
}
