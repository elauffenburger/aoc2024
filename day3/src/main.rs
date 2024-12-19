use lazy_static::lazy_static;

lazy_static! {
    static ref MUL_REGEX: Regex =
        Regex::new(r"mul\(([0-9]+?),([0-9]+?)\)").expect("expected compiled regex");
    static ref MUL_WITH_CONDITIONALS_REGEX: Regex =
        Regex::new(r"(mul\(([0-9]+?),([0-9]+?)\))|(do\(\))|(don't\(\))")
            .expect("expected compiled regex");
}

use regex::Regex;

use std::{
    error, fs,
    io::{self, Read, Write},
    str::from_utf8,
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

fn part_one(mut file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    // Combine all the lines into one.
    let buf = {
        let mut buf = vec![];
        file_reader.read_to_end(&mut buf)?;

        buf
    };

    let buf = from_utf8(&buf)?;

    // HACK: okay let's just regex this for now and we'll do a real parser if it comes up later!
    let mut result = 0;
    for captures in MUL_REGEX.captures_iter(buf) {
        let left = captures.get(1).unwrap().as_str().parse::<i32>()?;
        let right = captures.get(2).unwrap().as_str().parse::<i32>()?;

        result += left * right;
    }

    io::stdout().write_fmt(format_args!("{}", result))?;

    Ok(())
}

fn part_two(mut file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    // Combine all the lines into one.
    let buf = {
        let mut buf = vec![];
        file_reader.read_to_end(&mut buf)?;

        buf
    };

    let buf = from_utf8(&buf)?;

    // HACK: okay let's just regex this for now and we'll do a real parser if it comes up later!
    let mut result = 0;
    let mut mul_enabled = true;
    for captures in MUL_WITH_CONDITIONALS_REGEX.captures_iter(buf) {
        match captures.get(0).unwrap().as_str() {
            "don't()" => {
                mul_enabled = false;
                continue;
            }
            "do()" => {
                mul_enabled = true;
                continue;
            }
            _ => {}
        }

        if !mul_enabled {
            continue;
        }

        let left = captures.get(2).unwrap().as_str().parse::<i32>()?;
        let right = captures.get(3).unwrap().as_str().parse::<i32>()?;

        result += left * right;
    }

    io::stdout().write_fmt(format_args!("{}", result))?;

    Ok(())
}
