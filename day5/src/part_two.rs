use std::{
    error, fs,
    io::{self, BufRead, Write},
};

use crate::utils;

pub fn part_two(file_reader: io::BufReader<fs::File>) -> Result<(), Box<dyn error::Error>> {
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

        // If an update isn't in the right order, fix it and then add the new middle page num to the result.
        if !utils::update_is_valid(&update, &rules) {
            // Fix the update.
            let mut fixed_update = update.clone();

            // For each num in the update, figure out where it needs to be placed if it's in the wrong location
            // by swapping the num with the number it violates.
            let mut i = 0;
            while i < fixed_update.len() {
                let num = fixed_update.get(i).unwrap().clone();

                // If we perform a swap, this will change, but we still want to keep track of `i` separately
                // because we'll want to continue iterating from its position when we're done placing this number.
                let mut num_i = i;

                // If we didn't have to fix the number, then we can move on; otherwise, we need to evaluate the rules again for
                // the number that's in the location we're currently solving for.
                let mut did_fix_num = false;

                for j in i + 1..fixed_update.len() {
                    let next_num = fixed_update.get(j).unwrap().clone();

                    // Fix num by swapping it with next_num.
                    if !utils::is_update_num_valid(&rules, num, next_num) {
                        fixed_update.swap(num_i, j);
                        num_i = j;

                        did_fix_num = true;

                        println!("swapping {} and {}", num, next_num);
                        dbg!(&fixed_update);
                    }
                }

                if !did_fix_num {
                    i += 1;
                }
            }

            dbg!(&fixed_update);

            let middle_page_num = fixed_update.get(fixed_update.len() / 2).unwrap();
            result += middle_page_num;
        }
    }

    std::io::stdout().write_fmt(format_args!("{}", result))?;
    Ok(())
}
