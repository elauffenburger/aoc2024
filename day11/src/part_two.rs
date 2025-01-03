use std::io;
use std::{error, fs};

use crate::stones;

use crate::Cli;

pub fn part_two(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let mut rocks = stones::Stones::from_reader(file_reader);

    for i in 0..75 {
        if args.debug {
            println!(
                "blinked {} time{} (len: {})...",
                i,
                if i == 1 { "" } else { "s" },
                rocks.count()
            );
        }

        rocks.blink();
    }

    println!("{}", rocks.count());

    Ok(())
}
