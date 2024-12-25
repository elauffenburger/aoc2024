use std::{
    collections::HashSet,
    error,
    fmt::{Debug, Write},
    io,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GuardHeading {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
pub enum MapItem {
    Guard(GuardHeading),
    Obstacle,
    ProbeObstacle,
    Free,
}

#[derive(PartialEq, Eq, Hash)]
pub struct PositionAndHeading {
    pub position: (usize, usize),
    pub heading: GuardHeading,
}

#[derive(Clone)]
pub struct Map {
    map: Vec<Vec<MapItem>>,
    dims: (usize, usize),
    guard_pos: Option<(usize, usize)>,

    guard_visited_pos: HashSet<(usize, usize)>,
    guard_hit_obstacle_pos: Option<(usize, usize)>,
}

impl Map {
    pub fn dimensions(&self) -> (usize, usize) {
        self.dims
    }

    pub fn guard_visited_positions<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a (usize, usize)> + 'a> {
        Box::new(self.guard_visited_pos.iter())
    }

    pub fn guard_position(&self) -> Option<PositionAndHeading> {
        match self.guard_pos {
            None => None,
            Some(guard_pos) => Some(PositionAndHeading {
                position: guard_pos,
                heading: match &self.map[guard_pos.1][guard_pos.0] {
                    MapItem::Guard(heading) => heading.clone(),
                    _ => panic!("expected guard at guard position"),
                },
            }),
        }
    }

    pub fn guard_hit_obstacle_position(&self) -> &Option<(usize, usize)> {
        &self.guard_hit_obstacle_pos
    }

    pub fn from_reader<R: io::BufRead>(reader: R) -> Result<Self, Box<dyn error::Error>> {
        let mut map = vec![];
        let mut guard_position = None;

        for (y, line) in reader.lines().enumerate() {
            let line = line?;

            let mut row = vec![];
            for (x, ch) in line.chars().enumerate() {
                let item = match ch {
                    '#' => MapItem::Obstacle,
                    '.' => MapItem::Free,
                    h @ _ => {
                        let guard = MapItem::Guard(match h {
                            '^' => GuardHeading::Up,
                            '>' => GuardHeading::Right,
                            'v' => GuardHeading::Down,
                            '<' => GuardHeading::Left,
                            _ => return Err(format!("unexpected guard heading {}", h).into()),
                        });

                        guard_position = Some((x, y));

                        guard
                    }
                };

                row.push(item);
            }

            map.push(row);
        }

        let dims = match map.len() {
            0 => (0, 0),
            len @ _ => (map[0].len(), len),
        };

        Ok(Map {
            map,
            dims,
            guard_pos: guard_position.clone(),
            guard_visited_pos: match guard_position {
                Some(position) => HashSet::from([position]),
                None => HashSet::new(),
            },
            guard_hit_obstacle_pos: None,
        })
    }

    pub fn items(&self) -> MapIterator {
        MapIterator {
            guard_map: &self,
            dims: self.dims,
            pos: None,
            done: false,
        }
    }

    pub fn tick(&mut self) -> Result<bool, Box<dyn error::Error>> {
        if self.guard_pos.is_none() {
            return Ok(false);
        }

        // Reset state from last tick.
        self.guard_hit_obstacle_pos = None;

        // Figure out the new guard position:
        let guard_pos = self.guard_pos.unwrap();
        let guard_heading = match &self.map[guard_pos.1][guard_pos.0] {
            MapItem::Guard(heading) => heading.clone(),
            _ => return Err(format!("expected guard at {:?}", guard_pos).into()),
        };

        // Figure out our guard's new position if they can continue going in the current heading.
        let proposed_guard_pos = match guard_heading {
            GuardHeading::Up => (guard_pos.0 as i32, guard_pos.1 as i32 - 1),
            GuardHeading::Right => (guard_pos.0 as i32 + 1, guard_pos.1 as i32),
            GuardHeading::Down => (guard_pos.0 as i32, guard_pos.1 as i32 + 1),
            GuardHeading::Left => (guard_pos.0 as i32 - 1, guard_pos.1 as i32),
        };

        // Check if the guard is going to go off map if they keep going.
        let guard_off_map = match proposed_guard_pos {
            (x, _) if x < 0 || x as usize >= self.map[0].len() => true,
            (_, y) if y < 0 || y as usize >= self.map.len() => true,
            _ => false,
        };

        // If the guard is going off map, change the guard_position to None and let the caller know
        // we're done!
        if guard_off_map {
            self.move_guard(None, &guard_heading);
            return Ok(false);
        }

        // ...otherwise, check if we're going to collide with an object; if so, rotate 90 degrees; otherwise,
        // go to that position!
        let proposed_guard_pos = (proposed_guard_pos.0 as usize, proposed_guard_pos.1 as usize);
        match &self.map[proposed_guard_pos.1][proposed_guard_pos.0] {
            MapItem::Obstacle | MapItem::ProbeObstacle => {
                let new_guard_heading = match guard_heading {
                    GuardHeading::Up => GuardHeading::Right,
                    GuardHeading::Right => GuardHeading::Down,
                    GuardHeading::Down => GuardHeading::Left,
                    GuardHeading::Left => GuardHeading::Up,
                };

                // Mark that we hit this obstacle.
                self.guard_hit_obstacle_pos = Some(proposed_guard_pos);

                // Move the guard.
                self.move_guard(Some(guard_pos), &new_guard_heading);
            }
            MapItem::Free => self.move_guard(Some(proposed_guard_pos), &guard_heading),
            MapItem::Guard(_) => {
                return Err(format!("somehow tried to move the guard into another guard").into())
            }
        }

        Ok(true)
    }

    fn move_guard(&mut self, position: Option<(usize, usize)>, heading: &GuardHeading) {
        // Free up the old position if the guard was on the map.
        if let Some(old_position) = self.guard_pos {
            self.map[old_position.1][old_position.0] = MapItem::Free;
        }

        // Update the map and record the position as visited if the guard is still on the map.
        if let Some(position) = position {
            self.map[position.1][position.0] = MapItem::Guard(heading.clone());

            // Record the position in the list of visited positions.
            self.guard_visited_pos.insert(position);
        }

        // Update the guard position.
        self.guard_pos = position;
    }

    pub fn set_item(
        &mut self,
        position: (usize, usize),
        item: MapItem,
    ) -> Result<(), Box<dyn error::Error>> {
        if position.1 >= self.map.len() {
            return Err(format!("y {} out of range ({})", position.1, self.map.len()).into());
        }

        if self.map[position.1].len() <= position.0 {
            return Err(format!(
                "x {} out of range ({})",
                position.0,
                self.map[position.1].len()
            )
            .into());
        }

        self.map[position.1][position.0] = item;

        Ok(())
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                let ch = match item {
                    MapItem::Guard(heading) => match heading {
                        GuardHeading::Up => '^',
                        GuardHeading::Right => '>',
                        GuardHeading::Down => 'v',
                        GuardHeading::Left => '<',
                    },
                    MapItem::Obstacle => '#',
                    MapItem::ProbeObstacle => 'O',
                    MapItem::Free =>
                    // If this spot has been visited, mark it; otherwise, show it as free.
                    {
                        if self.guard_visited_pos.contains(&(x, y)) {
                            'X'
                        } else {
                            '.'
                        }
                    }
                };

                f.write_char(ch)?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

pub struct MapIterator<'a> {
    guard_map: &'a Map,
    dims: (usize, usize),

    pos: Option<(usize, usize)>,
    done: bool,
}

impl<'a> MapIterator<'a> {
    pub fn curr(&self) -> Option<(usize, usize)> {
        if self.done || self.pos.is_none() {
            None
        } else {
            Some(self.pos.unwrap())
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a MapItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Go to the next col.
        let mut new_pos = match self.pos {
            None => (0, 0),
            Some(pos) => (pos.0 + 1, pos.1),
        };

        // If we've completed this row, wrap around to the next row.
        if new_pos.0 >= self.dims.0 {
            new_pos = (0, new_pos.1 + 1);

            // If we've completed all rows, we're done!
            if new_pos.1 >= self.dims.1 {
                self.done = true;
                self.pos = None;

                return None;
            }
        }

        // Keep track of the new current position.
        self.pos = Some(new_pos);

        Some(&self.guard_map.map[new_pos.1][new_pos.0])
    }
}
