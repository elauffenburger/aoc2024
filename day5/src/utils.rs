use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io,
};

pub fn read_rules<R: io::BufRead>(
    lines: &mut io::Lines<R>,
) -> Result<HashMap<u32, HashSet<u32>>, Box<dyn Error>> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line?;

        if line == "" {
            break;
        }

        let (num, before_num) = line
            .split_once('|')
            .map(|kvp| {
                let key = kvp.0.parse::<u32>().expect("expected u32 for key");
                let val = kvp.1.parse::<u32>().expect("expected u32 for val");

                (key, val)
            })
            .expect("expected rule");

        match rules.get_mut(&num) {
            Some(before_rules) => {
                before_rules.insert(before_num);
            }
            None => {
                let mut before_rules = HashSet::new();
                before_rules.insert(before_num);

                rules.insert(num, before_rules);
            }
        };
    }
    Ok(rules)
}

pub fn update_is_valid(update: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    // for each num:
    for i in 0..update.len() {
        let num = update.get(i).unwrap();

        // for each next_num after num:
        for j in i + 1..update.len() {
            let next_num = update.get(j).unwrap();

            if !is_update_num_valid(rules, *num, *next_num) {
                return false;
            }
        }
    }

    return true;
}

pub fn is_update_num_valid(rules: &HashMap<u32, HashSet<u32>>, num: u32, next_num: u32) -> bool {
    // if num is in next_num's list of before_nums, fail
    match rules.get(&next_num) {
        None => true,
        Some(rules_for_next_num) => !rules_for_next_num.contains(&num),
    }
}
