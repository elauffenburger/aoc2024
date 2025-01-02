use std::{collections::HashSet, error, io::BufRead};

pub struct Map {
    map: Vec<Vec<u32>>,
    trailheads: Vec<Location>,
}

pub type Location = (usize, usize);

impl Map {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Map, Box<dyn error::Error>> {
        let mut map = vec![];
        let mut trailheads = vec![];

        for (y, line) in reader.lines().enumerate() {
            let line = line?;

            let mut row = vec![];
            for (x, ch) in line.chars().enumerate() {
                let val = ch
                    .to_digit(10)
                    .expect(format!("expected u32 at ({}, {})", x, y).as_str());

                // If this is a trailhead, remember it for later.
                if val == 0 {
                    trailheads.push((x, y))
                }

                row.push(val);
            }

            map.push(row);
        }

        Ok(Map { map, trailheads })
    }

    pub fn trailheads(&self) -> &[Location] {
        &self.trailheads
    }

    pub fn next_steps(
        &self,
        loc: &Location,
    ) -> Result<Vec<(Location, u32)>, Box<dyn error::Error>> {
        let loc_val = self.try_get(&(loc.0 as i32, loc.1 as i32)).ok_or(format!(
            "expected ({}, {}) to be a valid location",
            loc.0, loc.1
        ))?;

        let loc = (loc.0 as i32, loc.1 as i32);

        let possible_steps = [
            (loc.0, loc.1 + 1),
            (loc.0 + 1, loc.1),
            (loc.0, loc.1 - 1),
            (loc.0 - 1, loc.1),
        ];

        let mut steps = vec![];
        for step in possible_steps {
            match self.try_get(&step) {
                Some(step_val) if step_val == loc_val + 1 => {
                    steps.push(((step.0 as usize, step.1 as usize), step_val));
                }
                _ => {}
            }
        }

        Ok(steps)
    }

    pub fn try_get(&self, maybe_loc: &(i32, i32)) -> Option<u32> {
        if maybe_loc.0 < 0 || maybe_loc.1 < 0 {
            return None;
        }

        let loc = (maybe_loc.0 as usize, maybe_loc.1 as usize);
        self.map
            .get(loc.1)
            .and_then(|row| row.get(loc.0))
            .map(|val| *val)
    }
}

pub fn find_paths<C: MutCollection<Location>>(args: &crate::Cli, map: &Map) -> C {
    let mut paths = C::new();

    // for each trailhead:
    for (trailhead_i, trailhead) in map.trailheads().iter().enumerate() {
        // find all paths for this trailhead!
        let mut frontier = vec![vec![*trailhead]];

        let mut trailhead_paths = C::new();
        while let Some(path) = frontier.pop() {
            let loc = path.last().unwrap();

            if args.debug {
                println!(
                    "path: {:?} ({})",
                    path,
                    map.try_get(&(loc.0 as i32, loc.1 as i32)).unwrap()
                );
            }

            // find all moves at this location:
            for (step_loc, val) in map.next_steps(loc).unwrap() {
                if args.debug {
                    println!("step: {:?} ({})", step_loc, val);
                }

                // if the val is 9, we found a complete path!
                if val == 9 {
                    if args.debug {
                        println!("found path!")
                    }

                    trailhead_paths.add(step_loc);
                    continue;
                }

                // otherwise, add the new path to the frontier and we'll check it out later!
                let mut new_path = path.clone();
                new_path.extend_from_slice(&[step_loc]);
                frontier.push(new_path);
            }
        }

        if args.debug {
            println!(
                "trailhead {} had {} paths",
                trailhead_i,
                trailhead_paths.len()
            );
        }

        paths.extend(trailhead_paths);
    }

    paths
}

pub trait MutCollection<T>: Extend<T> + IntoIterator<Item = T> {
    fn add(&mut self, item: T);
    fn new() -> Self;
    fn len(&self) -> usize;
}

impl<T> MutCollection<T> for Vec<T> {
    fn add(&mut self, item: T) {
        self.push(item);
    }

    fn new() -> Self {
        Vec::new()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> MutCollection<T> for HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    fn add(&mut self, item: T) {
        self.insert(item);
    }

    fn new() -> Self {
        Self::new()
    }

    fn len(&self) -> usize {
        self.len()
    }
}
