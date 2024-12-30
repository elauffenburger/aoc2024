use std::{
    collections::HashSet,
    error, fs,
    io::{self},
};

use crate::{grid::*, Cli};

pub fn part_two(
    _: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let grid = Grid::from_reader(file_reader)?;
    let mut antinodes = HashSet::new();

    // For each antenna, calculate the position of antinodes formed by antennas of the same frequency.
    for (_, antennas) in grid.antennas.iter() {
        for (i, antenna_a) in antennas.iter().enumerate() {
            let antenna_a = (antenna_a.0 as i32, antenna_a.1 as i32);

            for antenna_b in &antennas[i + 1..] {
                let antenna_b = (antenna_b.0 as i32, antenna_b.1 as i32);

                // figure out which node is on the left so we can use that as our reference.
                let (left, right) = if antenna_a.0 < antenna_b.0 {
                    (antenna_a, antenna_b)
                } else {
                    (antenna_b, antenna_a)
                };

                // get the distance between the two points.
                //
                // we're assuming here that we're in a top-left (vs bottom-left) orientation here.
                let distance = (right.0 - left.0, right.1 - left.1);

                // create our antinodes by starting drawing a line through the grid that passes between the two points.

                // add antinodes starting at the left antenna and going away from the right antenna
                {
                    let mut curr_loc = left.clone();
                    while grid.is_valid_location(curr_loc) {
                        antinodes.insert(curr_loc);

                        curr_loc.0 -= distance.0;
                        curr_loc.1 -= distance.1;
                    }
                }

                // add antinodes starting at the left antenna and going through the right antenna
                {
                    let mut curr_loc = left.clone();
                    while grid.is_valid_location(curr_loc) {
                        antinodes.insert(curr_loc);

                        curr_loc.0 += distance.0;
                        curr_loc.1 += distance.1;
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());

    Ok(())
}
