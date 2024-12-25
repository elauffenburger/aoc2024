use std::{
    error, fs,
    io::{self, BufRead, Write},
};

use crate::utils;

pub fn part_one(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
    let mut lines = file_reader.lines();

    // Read the ruleset.
    let rules = utils::read_rules(&mut lines)?;

    // Verify each update.
    let mut result = 0;
    for line in lines {
        let line = line?;

        let update: Vec<u32> = line
            .split(',')
            .map(|num| {
                num.parse()
                    .expect(format!("expected {} to be u32 in update", num).as_str())
            })
            .collect();

        // Verify each num in the update using the ruleset.
        if utils::update_is_valid(&update, &rules) {
            let middle_page_num = update.get(update.len() / 2).unwrap();
            result += middle_page_num;
        }
    }

    std::io::stdout().write_fmt(format_args!("{}", result))?;
    Ok(())
}
