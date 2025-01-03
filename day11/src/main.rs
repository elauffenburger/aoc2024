use std::{
    error, fs,
    io::{self},
};

use clap::{Parser, ValueEnum};

mod part_one;
use part_one::part_one;

mod part_two;
use part_two::part_two;

mod stones;

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

    #[arg(long = "debug", default_value_t = false)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    let input_path = cli.input.clone().unwrap_or("input".into());
    let file_reader = io::BufReader::new(fs::File::open(input_path)?);

    match cli.part {
        DayPart::One => part_one(&cli, file_reader),
        DayPart::Two => part_two(&cli, file_reader),
    }
}
