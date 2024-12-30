use std::{
    error, fs,
    io::{self, BufRead, Write},
};

use crate::grid_getter::GridGetter;

pub fn part_two(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
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
            let getter = GridGetter::new(&grid, row as u32, col as u32);

            if match_tl_br(&getter) && match_tr_bl(&getter) {
                count += 1;
            }
        }
    }

    io::stdout().write_fmt(format_args!("{}", count))?;

    Ok(())
}

fn match_tl_br(grid: &GridGetter) -> bool {
    match (grid.get(-1, 1), grid.get(0, 0), grid.get(1, -1)) {
        (Some('M'), Some('A'), Some('S')) => true,
        (Some('S'), Some('A'), Some('M')) => true,
        _ => false,
    }
}

fn match_tr_bl(grid: &GridGetter) -> bool {
    match (grid.get(1, 1), grid.get(0, 0), grid.get(-1, -1)) {
        (Some('M'), Some('A'), Some('S')) => true,
        (Some('S'), Some('A'), Some('M')) => true,
        _ => false,
    }
}
