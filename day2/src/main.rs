use std::{
    error, fs,
    io::{self, BufRead, Write},
};

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DayPart {
    One,
    Two,
}

#[derive(Parser)]
#[command()]
struct Cli {
    #[arg(long = "part", value_enum)]
    part: DayPart,

    #[arg(long = "input")]
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    let input_path = cli.input.unwrap_or("input".into());
    let file_reader = io::BufReader::new(fs::File::open(input_path)?);

    match cli.part {
        DayPart::One => part_one(file_reader),
        DayPart::Two => part_two(file_reader),
    }
}

fn part_one(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    let mut valid_reports = 0;

    let mut lines = file_reader.lines();
    while let Some(line) = lines.next() {
        let report = line?
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("expected u32"))
            .collect::<Vec<u32>>();

        if is_report_valid(&report)? {
            valid_reports += 1;
        }
    }

    io::stdout().write_fmt(format_args!("{}", valid_reports))?;
    Ok(())
}

fn is_report_valid(report: &[u32]) -> Result<bool, Box<dyn error::Error>> {
    // for each report:
    //   keep track of:
    //     if all numbers are increasing
    //     if all numbers are decreasing
    //     the last number
    //   for each num:
    //     if not(1 <= abs(num-last_num) <= 3), fail
    //   if not(all_increasing) or not(all_decreasing), fail
    //   otherwise, pass!

    let (mut all_increasing, mut all_decreasing) = (true, true);
    let mut last_num = None;
    for num in report {
        if last_num.is_none() {
            last_num = Some(num);
            continue;
        }

        {
            let last_num = last_num.unwrap();
            let abs_diff = last_num.abs_diff(*num);

            if !(abs_diff >= 1 && abs_diff <= 3) {
                return Ok(false);
            }

            if num <= last_num {
                all_increasing = false;
            } else if num >= last_num {
                all_decreasing = false;
            }
        }

        last_num = Some(num);
    }

    return Ok(all_increasing || all_decreasing);
}

fn part_two(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    let mut valid_reports = 0;

    let mut lines = file_reader.lines();
    'line_loop: while let Some(line) = lines.next() {
        let report = line?
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("expected u32"))
            .collect::<Vec<u32>>();

        // Check if the report is valid without any modifications.
        if is_report_valid(&report)? {
            valid_reports += 1;
            continue 'line_loop;
        }

        // Try every permutation of dropping a number from the report; if any
        // permutation is valid, then the entire report is considered valid.
        for i in 0..report.len() {
            let permutation = [&report[0..i], &report[i + 1..]].concat();

            if is_report_valid(&permutation)? {
                valid_reports += 1;
                continue 'line_loop;
            }
        }
    }

    io::stdout().write_fmt(format_args!("{}", valid_reports))?;
    Ok(())
}
