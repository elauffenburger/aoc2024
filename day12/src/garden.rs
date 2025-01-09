use std::{collections::HashSet, error, io::BufRead};

use maplit::hashset;

pub struct Garden {
    grid: Vec<Vec<char>>,
}

impl Garden {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self, Box<dyn error::Error>> {
        let mut grid = vec![];

        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            let line = line?;

            let mut row = vec![];
            for ch in line.chars() {
                row.push(ch);
            }

            grid.push(row)
        }

        Ok(Self { grid })
    }

    pub fn regions(&self) -> Vec<GardenRegion> {
        let mut regions: Vec<GardenRegion> = vec![];

        // Keep track of plots we've already associated with a region.
        let mut marked = HashSet::new();

        // Iterate over each plot until we find a cell we haven't associated with a region.
        let mut iter = self.iter();
        while let Some(curr) = iter.next() {
            let curr = (curr.0 as i32, curr.1 as i32);
            if marked.contains(&curr) {
                continue;
            }

            let region_name = self.try_get(curr).unwrap();
            let (mut min_x, mut min_y) = (None, None);
            let (mut max_x, mut max_y) = (None, None);

            // Now that we've found a plot that we haven't marked yet, run a search
            // to find all neighboring plots of the same name.
            //
            // We can do that by continously expanding a frontier that will find plots of the
            // same name.
            let mut frontier = vec![curr];
            let mut seen = hashset![curr];

            let mut region_perimeter = 0;
            let mut region_area = 0;
            while !frontier.is_empty() {
                let plot = frontier.pop().unwrap();
                marked.insert(plot);
                region_area += 1;

                let (x, y) = plot;

                // Do any sort of min/max memorizations so we can keep track of the bounds of this region.
                min_x.replace_if(x, |val| x < *val);
                max_x.replace_if(x, |val| x > *val);
                min_y.replace_if(y, |val| y < *val);
                max_y.replace_if(y, |val| y > *val);

                // Check each of the neighbors to see if they're part of the contiguous region.
                let neighbors = [(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)];

                for neighbor in neighbors {
                    match self.try_get(neighbor) {
                        None => region_perimeter += 1,
                        Some(neighbor_name) => {
                            if *neighbor_name == *region_name {
                                if !seen.contains(&neighbor) {
                                    seen.insert(neighbor);
                                    frontier.push(neighbor);
                                }
                            } else {
                                region_perimeter += 1;
                            }
                        }
                    }
                }
            }

            regions.push(GardenRegion {
                name: *region_name,
                area: region_area,
                perimeter: region_perimeter,
                bounds: Bounds {
                    top_left: (min_x.unwrap() as usize, min_y.unwrap() as usize),
                    bottom_right: (max_x.unwrap() as usize, max_y.unwrap() as usize),
                },
                plots: seen.into_iter().collect(),
            });
        }

        regions
    }

    pub fn try_get(&self, point: (i32, i32)) -> Option<&char> {
        let (x, y) = point;

        if x < 0 || y < 0 {
            return None;
        }

        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    pub fn iter(&self) -> BoundsIterator {
        BoundsIterator {
            curr: None,
            bounds: Bounds {
                top_left: (0, 0),
                bottom_right: (self.grid[0].len() - 1, self.grid.len() - 1),
            },
        }
    }
}

#[derive(Debug)]
pub struct GardenRegion {
    pub name: char,
    pub area: usize,
    pub perimeter: usize,
    pub bounds: Bounds,
    pub plots: HashSet<(i32, i32)>,
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub top_left: (usize, usize),
    pub bottom_right: (usize, usize),
}

impl Bounds {
    pub fn iter(&self) -> BoundsIterator {
        BoundsIterator {
            curr: None,
            bounds: self.clone(),
        }
    }
}

pub struct BoundsIterator {
    curr: Option<(usize, usize)>,
    bounds: Bounds,
}

impl Iterator for BoundsIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // If we haven't started yet, start at the top-left.
        if let None = self.curr {
            self.curr = Some(self.bounds.top_left.clone());
            return self.curr;
        }

        let curr = self.curr.as_mut().unwrap();

        // If we can travel right, do it!
        if curr.0 + 1 <= self.bounds.bottom_right.0 {
            curr.0 += 1;

            return self.curr;
        }

        // Otherwise, if we can travel down, do it and move x back to min_x.
        if curr.1 + 1 <= self.bounds.bottom_right.1 {
            curr.1 += 1;
            curr.0 = self.bounds.top_left.0;

            return self.curr;
        }

        // Otherwise, we're done.
        None
    }
}

trait ReplaceIf {
    type Item;

    fn replace_if<F: FnOnce(&Self::Item) -> bool>(&mut self, item: Self::Item, condition: F);
}

impl<T> ReplaceIf for Option<T> {
    type Item = T;

    fn replace_if<F: FnOnce(&Self::Item) -> bool>(&mut self, item: Self::Item, condition: F) {
        if self.is_none() {
            self.replace(item);
            return;
        }

        if let Some(val) = self {
            if condition(val) {
                self.replace(item);
            }
        }
    }
}
