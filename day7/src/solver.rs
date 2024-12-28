use std::{
    error, fs,
    io::{self, BufRead},
};

use crate::Cli;

pub fn solve(
    _: &Cli,
    file_reader: io::BufReader<fs::File>,
    operators: &[Operator],
) -> Result<(), Box<dyn error::Error>> {
    let mut result = 0;

    for line in file_reader.lines() {
        let line = line?;

        let (eq_total, eq_nums) = {
            let (total, right) = line
                .split_once(":")
                .ok_or("expected colon-delimited input line")?;

            let eq_nums = right[1..]
                .split(" ")
                .map(|num| {
                    num.parse()
                        .expect("expected numbers on right side of colon to be i64s")
                })
                .collect::<Vec<i64>>();

            (total.parse::<i64>()?, eq_nums)
        };

        // Iterate over each permutation of operator placement for numbers and try to find one that solves the equation.
        if try_solve(operators, eq_total, &eq_nums[1..], eq_nums[0]) {
            result += eq_total;
        }
    }

    println!("{}", result);

    Ok(())
}

fn try_solve(operators: &[Operator], expected: i64, nums: &[i64], acc: i64) -> bool {
    match nums {
        [] => return acc == expected,
        [num, rest @ ..] => {
            for op in operators {
                if try_solve(operators, expected, rest, op.exec(&acc, num)) {
                    return true;
                }
            }
        }
    }

    return false;
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    pub fn exec(&self, left: &i64, right: &i64) -> i64 {
        match self {
            Operator::Add => left + right,
            Operator::Mul => left * right,
            Operator::Cat => format!("{}{}", left, right).parse().unwrap(),
        }
    }
}
