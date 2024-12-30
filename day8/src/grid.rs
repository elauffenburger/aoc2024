use std::{collections::HashMap, error, io::BufRead};

pub enum GridItem {
    Empty,
    Antenna { _freq: char },
}

pub struct Grid {
    pub grid: Vec<Vec<GridItem>>,

    pub antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Grid {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self, Box<dyn error::Error>> {
        let mut rows = vec![];
        let mut antennas = HashMap::new();

        for (y, line) in reader.lines().enumerate() {
            let line = line?;

            let mut row = vec![];

            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => row.push(GridItem::Empty),
                    freq @ _ => {
                        match antennas.get_mut(&freq) {
                            None => {
                                antennas.insert(freq, vec![(x, y)]);
                            }
                            Some(antennas) => {
                                antennas.push((x, y));
                            }
                        }

                        row.push(GridItem::Antenna { _freq: freq });
                    }
                }
            }

            rows.push(row);
        }

        Ok(Grid {
            grid: rows,
            antennas,
        })
    }

    pub fn is_valid_location(&self, loc: (i32, i32)) -> bool {
        (loc.0 >= 0 && loc.0 < self.grid[0].len() as i32)
            && (loc.1 >= 0 && loc.1 < self.grid.len() as i32)
    }
}
