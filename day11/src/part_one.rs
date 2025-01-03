use std::{
    error, fs,
    io::{self},
};

use crate::stones;

use crate::Cli;

pub fn part_one(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let mut stones_list = stones::Stones::from_reader(file_reader);

    if args.debug {
        println!("{:?}", stones_list.distinct());
    }

    for _ in 0..6 {
        stones_list.blink();

        if args.debug {
            println!("{:?}", stones_list.distinct());
        }
    }

    println!("{}", stones_list.count());

    Ok(())
}
