use std::{
    collections::HashSet,
    error, fs,
    io::{self},
};

use crate::{
    guard_map::{self, PositionAndHeading},
    Cli,
};

pub fn part_two(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let source_map = guard_map::Map::from_reader(file_reader)?;

    let mut map_items = source_map.items();
    let mut num_loops = 0u32;

    // for each free index in the map:
    let mut i = 0;
    let dims = {
        let dims = source_map.dimensions();
        dims.0 * dims.1
    };
    while let Some(item) = map_items.next() {
        println!("item: {}/{}", i, dims);

        match item {
            guard_map::MapItem::Free => {
                // Create a copy of the map.
                let mut curr_map = source_map.clone();

                // Keep track of the hit obstacles.
                let mut hit_obstacles_with_heading = HashSet::new();

                // place an obstacle in that index
                curr_map.set_item(map_items.curr().unwrap(), guard_map::MapItem::ProbeObstacle)?;

                // while the guard hasn't gone off the map, tick the map!
                while curr_map.tick()? {
                    // check if the guard has hit the obstacle
                    let hit_obstacle = curr_map.guard_hit_obstacle_position();
                    if hit_obstacle.is_none() {
                        continue;
                    }

                    let hit_obstacle = hit_obstacle.unwrap();
                    let guard_pos_and_heading = curr_map.guard_position().unwrap();

                    // Construct a key that represents hitting this obstacle with this guard heading.
                    let obstacle_pos_and_heading = PositionAndHeading {
                        position: hit_obstacle,
                        heading: guard_pos_and_heading.heading,
                    };

                    // if the guard has already hit the obstacle with the same heading, we found a loop!
                    if hit_obstacles_with_heading.contains(&obstacle_pos_and_heading) {
                        if args.debug {
                            println!("already hit obstacle!");
                            println!("{:?}", &curr_map);
                        }

                        num_loops += 1;
                        break;
                    }

                    // ...otherwise, record that the guard has hit this obstacle with this heading
                    hit_obstacles_with_heading.insert(obstacle_pos_and_heading);
                }
            }
            _ => continue,
        }

        i += 1;
    }

    println!("{}", num_loops);

    Ok(())
}
