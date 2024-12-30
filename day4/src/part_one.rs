use std::{
    error, fs,
    io::{self, BufRead, Write},
};

use crate::grid_getter::GridGetter;

pub fn part_one(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    let grid = {
        let mut grid = vec![];

        let mut lines = file_reader.lines();
        while let Some(line) = lines.next() {
            let line = line?;
            grid.push(line.chars().collect::<Vec<char>>());
        }

        grid
    };

    let mut count = 0;

    // for each character:
    //   check up, up+forward, forward, forward+down, down, down+back, back, back+up and increment the count for each one that matches
    for row in 0..grid.len() {
        for col in 0..grid.get(row).unwrap().len() {
            let grid = GridGetter::new(&grid, row as u32, col as u32);

            // up
            if grid.seq_matches(&[(0, 0), (0, 1), (0, 2), (0, 3)], "XMAS") {
                count += 1;
            }

            // up-forward
            if grid.seq_matches(&[(0, 0), (1, 1), (2, 2), (3, 3)], "XMAS") {
                count += 1;
            }

            // forward
            if grid.seq_matches(&[(0, 0), (1, 0), (2, 0), (3, 0)], "XMAS") {
                count += 1;
            }

            // forward_down
            if grid.seq_matches(&[(0, 0), (1, -1), (2, -2), (3, -3)], "XMAS") {
                count += 1;
            }

            // down
            if grid.seq_matches(&[(0, 0), (0, -1), (0, -2), (0, -3)], "XMAS") {
                count += 1;
            }

            // down-back
            if grid.seq_matches(&[(0, 0), (-1, -1), (-2, -2), (-3, -3)], "XMAS") {
                count += 1;
            }

            // back
            if grid.seq_matches(&[(0, 0), (-1, 0), (-2, 0), (-3, 0)], "XMAS") {
                count += 1;
            }

            // back-up
            if grid.seq_matches(&[(0, 0), (-1, 1), (-2, 2), (-3, 3)], "XMAS") {
                count += 1;
            }
        }
    }

    io::stdout().write_fmt(format_args!("{}", count))?;

    Ok(())
}
