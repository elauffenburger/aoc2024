use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    mem,
};

use maplit::hashmap;

pub struct Stones {
    stones: HashSet<u64>,

    curr_blink_counts: HashMap<u64, u64>,
    splits: HashMap<u64, (u64, Option<u64>)>,
}

impl Stones {
    pub fn from_reader<R: BufRead>(reader: R) -> Self {
        let mut stones = HashSet::new();
        let mut counts = HashMap::new();

        let line = reader.lines().next().unwrap().unwrap();
        for stone in line.split(" ").map(|ch| ch.parse::<u64>().unwrap()) {
            counts.insert(
                stone,
                match counts.get(&stone) {
                    Some(count) => count + 1,
                    None => 1,
                },
            );

            stones.insert(stone);
        }

        Stones {
            stones,
            curr_blink_counts: counts,
            splits: hashmap! {
                0 => (1, None),
            },
        }
    }

    pub fn blink(&mut self) {
        // Swap out the existing set of stones for a new empty one with the same capacity.
        let mut stones = HashSet::with_capacity(self.stones.len());
        mem::swap(&mut stones, &mut self.stones);

        // Swap out the blink counts from last blink.
        let mut last_blink_counts = HashMap::with_capacity(self.curr_blink_counts.capacity());
        mem::swap(&mut last_blink_counts, &mut self.curr_blink_counts);

        for stone in stones {
            // Get the number of stones with this value from last blink.
            let stone_count = last_blink_counts
                .get(&stone)
                .expect("a stone from last blink should have a count")
                .to_owned();

            let (left, right) = self.split(stone);

            // Add produced stone n-many times since each parent would have _also_ produced these stones.
            self.add_stone(left, stone_count);
            if let Some(right) = right {
                self.add_stone(right, stone_count);
            }
        }
    }

    #[inline]
    fn add_stone(&mut self, stone: u64, count: u64) {
        // Add the stone.
        self.stones.insert(stone);

        // Save the total count of this stone value for this blink.
        let count = match self.curr_blink_counts.get(&stone) {
            Some(curr) => curr + count,
            None => count,
        };

        self.curr_blink_counts.insert(stone, count);
    }

    fn split(&mut self, stone: u64) -> (u64, Option<u64>) {
        // Check if we already know how this stone is going to split...
        if let Some(split) = self.splits.get(&stone) {
            return *split;
        }

        // ...otherwise, let's find out!
        let split = {
            let num_digits = num_digits(stone);

            // If the number of digits is even, split the number...
            if num_digits % 2 == 0 {
                let (left, right) = split_num(stone, num_digits);

                (left, Some(right))
            } else {
                // ...otherwise, just multiply by 2024
                (stone * 2024, None)
            }
        };

        // Record how this stone should split.
        self.splits.insert(stone, split);

        split
    }

    pub fn count(&self) -> u64 {
        self.curr_blink_counts
            .iter()
            .fold(0, |acc, kvp| acc + *kvp.1)
    }

    pub fn distinct(&self) -> &HashSet<u64> {
        &self.stones
    }
}

fn num_digits(num: u64) -> usize {
    let mut res = 0;

    let mut curr = num;
    while curr > 0 {
        curr /= 10;
        res += 1;
    }

    res
}

fn split_num(num: u64, digits: usize) -> (u64, u64) {
    // Get the left divisor.
    // e.g. num=5214; digits=4; 10^(4/2) = 100; left = 5214 / 100 = 52; right = 5214 % 100 = 14.
    let left_divisor = 10u64.pow((digits / 2) as u32);

    let left = num / left_divisor;
    let right = num % left_divisor;

    (left, right)
}
