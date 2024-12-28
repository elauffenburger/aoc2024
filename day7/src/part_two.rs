use std::{
    error, fs,
    io::{self},
};

use crate::{solver, Cli};

pub fn part_two(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    solver::solve(
        args,
        file_reader,
        &[
            solver::Operator::Add,
            solver::Operator::Mul,
            solver::Operator::Cat,
        ],
    )
}
