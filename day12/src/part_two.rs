use std::io;
use std::{error, fs};

use crate::{Cli, Garden, GardenRegion};

pub fn part_two(
    args: &Cli,
    file_reader: io::BufReader<fs::File>,
) -> Result<(), Box<dyn error::Error>> {
    let garden = crate::Garden::from_reader(file_reader)?;

    let mut total = 0;

    let regions = garden.regions();
    for region in regions {
        // Find the number of sides for each region by performing scans horizontally and vertically.
        // Below is the algorithm for the horizontal scan, but the vertical scan would just be the inverse in terms of directions and orientation.
        //
        // for each plot:
        //   if the plot doesn't belong to the region:
        //     reset top state
        //     reset bottom state
        //   else:
        //     if the plot above is part of the region:
        //       if tracking top:
        //         tracking_top = false
        //     else:
        //       if top is not being tracked:
        //         num_sides++
        //         tracking_top = true
        //
        //     if the plot below is part of the region:
        //       if tracking bottom:
        //         tracking_bottom = false
        //     else:
        //       if bottom is not being tracked:
        //         num_sides++
        //         tracking_bottom = true

        let num_sides = scan_for_sides(args, &garden, &region, ScanDirection::Horizontal)
            + scan_for_sides(args, &garden, &region, ScanDirection::Vertical);

        if args.debug {
            println!(
                "region={} area={} sides={}",
                region.name, region.area, num_sides
            );
        }

        total += num_sides * region.area as u32;
    }

    println!("{}", total);

    Ok(())
}

fn scan_for_sides(
    args: &Cli,
    garden: &Garden,
    region: &GardenRegion,
    direction: ScanDirection,
) -> u32 {
    let mut num_sides = 0;

    let x_range = (region.bounds.top_left.0, region.bounds.bottom_right.0);
    let y_range = (region.bounds.top_left.1, region.bounds.bottom_right.1);

    let (outer_range, inner_range) = match direction {
        ScanDirection::Horizontal => (y_range, x_range),
        ScanDirection::Vertical => (x_range, y_range),
    };

    for i in outer_range.0..=outer_range.1 {
        let mut tracking = [false, false];

        for j in inner_range.0..=inner_range.1 {
            let plot = match direction {
                ScanDirection::Horizontal => (j as i32, i as i32),
                ScanDirection::Vertical => (i as i32, j as i32),
            };

            let plot_region = garden.try_get(plot).unwrap();

            // If this plot has the same region name but isn't _actually_ part of this region,
            // just ignore it! It's probably in an isolated island somewhere.
            if *plot_region == region.name && !region.plots.contains(&plot) {
                continue;
            }

            if args.debug {
                println!("plot: {:?}", &plot);
            }

            if *plot_region != region.name {
                if args.debug && tracking[0] {
                    println!("stopped tracking 0 at {:?}", &plot);
                }

                if args.debug && tracking[1] {
                    println!("stopped tracking 1 at {:?}", &plot);
                }

                tracking = [false, false];
                continue;
            }

            let neighbors = match direction {
                ScanDirection::Horizontal => [(plot.0, plot.1 - 1), (plot.0, plot.1 + 1)],
                ScanDirection::Vertical => [(plot.0 - 1, plot.1), (plot.0 + 1, plot.1)],
            };

            for i in 0..2 {
                let neighbor = &neighbors[i];

                if args.debug {
                    println!("checking {:?}", neighbor);
                }

                match garden.try_get(neighbors[i]) {
                    Some(name) if *name == region.name => {
                        if args.debug && tracking[i] {
                            println!("stopped tracking {} at {:?}", i, &plot);
                        }

                        tracking[i] = false
                    }
                    _ => {
                        if !tracking[i] {
                            if args.debug {
                                println!("started tracking {} at {:?}", i, &plot);
                            }

                            num_sides += 1;
                            tracking[i] = true;
                        }
                    }
                }
            }
        }
    }

    if args.debug {
        println!("num_sides ({:?}): {:?}", &direction, &num_sides);
    }

    num_sides
}

#[derive(Debug)]
pub enum ScanDirection {
    Horizontal,
    Vertical,
}
