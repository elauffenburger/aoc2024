use std::{
    collections::HashSet,
    error, fs,
    io::{self},
};

use crate::{grid::*, Cli};

pub fn part_one(
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

                // create two antinodes (one for each antenna).
                //
                // note that distance.1 _might_ be negative here, but all that indicates is that
                // the right antenna is above the left antenna.
                //
                // in either case, (left.1 - distance.1) and (right.1 + distance.1) are always going to get you
                // a y that is in the opposite direction of right!
                let formed_antinodes = [
                    (left.0 - distance.0, left.1 - distance.1),
                    (right.0 + distance.0, right.1 + distance.1),
                ];

                // add the antinodes!
                for antinode in formed_antinodes {
                    if grid.is_valid_location(antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());

    Ok(())
}
