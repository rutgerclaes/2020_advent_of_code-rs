extern crate log;
extern crate pretty_env_logger;

use itertools::Itertools;
use log::{debug, info, trace};

fn main() {
    pretty_env_logger::init();
    info!("--- [AoC 2020] Day 25: Combo Breaker ---");

    let card_public_key = 5764801;
    let door_public_key = 17807724;

    let card_public_key = 12578151;
    let door_public_key = 5051300;

    println!(
        "Solution to part one: {}",
        part_one(card_public_key, door_public_key)
    );
}

fn part_one(card: u64, door: u64) -> u64 {
    let result = (1..usize::MAX).try_fold(1, |value, loop_size| {
        trace!("Loop size: {}", loop_size);
        match (value * 7 as u64).rem_euclid(20201227) {
            res if res == card => {
                info!("Key found: {} at loop size {}", res, loop_size);
                Err(transform(loop_size, door))
            }
            res if res == door => {
                info!("Key found: {} at loop size {}", res, loop_size);
                Err(transform(loop_size, card))
            }
            value => Ok(value),
        }
    });

    match result {
        Err(encryption_key) => encryption_key,
        Ok(_) => panic!("Could not find encryption key"),
    }
}

fn transform(loop_size: usize, subject_number: u64) -> u64 {
    debug!("Trying loop size: {}", loop_size);
    (0..loop_size).fold(1, |value, _| (value * subject_number).rem_euclid(20201227))
}
