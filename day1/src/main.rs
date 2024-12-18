const NUM_IDS: usize = 1000;

use std::{
    collections::HashMap,
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
    // Read our file into left and right lists of ids.
    let (left, right) = {
        let mut left = Vec::with_capacity(NUM_IDS);
        let mut right = Vec::with_capacity(NUM_IDS);

        let mut lines = file_reader.lines();
        while let Some(line) = lines.next() {
            let line = line?;

            // each line is /^(?<left>[1-9][0-9]{4})   (?<right>[1-9][0-9]{4})$/
            let mut parts = line.split_whitespace().take(2);

            let left_id: u32 = parts.next().unwrap().parse()?;
            let right_id: u32 = parts.next().unwrap().parse()?;

            left.push(left_id);
            right.push(right_id);
        }

        // Sort lists.
        left.sort();
        right.sort();

        (left, right)
    };

    // zip left and right and sum the abs distance between each entry.
    let sum_of_distances = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right));

    std::io::stdout().write_fmt(format_args!("{}\n", sum_of_distances))?;
    Ok(())
}

fn part_two(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    // Read our file into left and right lists of ids.
    let (left_ids, right_counts) = {
        let mut left_ids = vec![];
        let mut right_counts = HashMap::new();

        let mut lines = file_reader.lines();
        while let Some(line) = lines.next() {
            let line = line?;

            // each line is /^(?<left>[1-9][0-9]{4})   (?<right>[1-9][0-9]{4})$/
            let mut parts = line.split_whitespace().take(2);

            let left_id: u32 = parts.next().unwrap().parse()?;
            let right_id: u32 = parts.next().unwrap().parse()?;

            left_ids.push(left_id);
            match right_counts.get(&right_id) {
                Some(count) => right_counts.insert(right_id, count + 1),
                None => right_counts.insert(right_id, 1),
            };
        }

        (left_ids, right_counts)
    };

    // calculate the similarity scores by iterating over the left list and finding the number of
    // times it appears in the right list.
    let sum_of_distances = left_ids.into_iter().fold(0, |acc, id| {
        let count_in_right = right_counts.get(&id).unwrap_or(&0);
        let similarity = id * count_in_right;

        acc + similarity
    });

    std::io::stdout().write_fmt(format_args!("{}\n", sum_of_distances))?;
    Ok(())
}
