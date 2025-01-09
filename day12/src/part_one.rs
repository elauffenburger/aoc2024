use std::{
    error, fs,
    io::{self},
};

use crate::Garden;

use crate::Cli;

pub fn part_one(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let garden = Garden::from_reader(file_reader)?;
    let regions = garden.regions();

    if args.debug {
        println!("{:?}", &regions);
    }

    let total_price: usize = regions
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum();

    println!("{}", total_price);

    Ok(())
}
