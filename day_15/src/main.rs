extern crate im_rc;

use im_rc::vector;
use im_rc::Vector;
use std::collections::HashMap;

fn main() {
    println!("--- [AoC 2020] Day 15: Rambunctious Recitation ---");

    // let test_input = vector!(0, 3, 6);
    let input = vector!(9, 12, 1, 4, 17, 0, 18);

    println!("Solution to part one: {}", part_one(&input));
    // println!("Solution to part two: {}", part_two(&input).unwrap());
}

fn part_one(input: &Vector<usize>) -> usize {
    let memory: HashMap<usize, usize> = input
        .iter()
        .enumerate()
        .filter(|(i, _)| i < &(input.len() - 1))
        .fold(HashMap::new(), |mut memory, (turn, number)| {
            memory.insert(*number, turn);
            memory
        });
    let (_, last_number) = ((input.len())..30000000).fold(
        (memory, *input.last().unwrap()),
        |(mut memory, last_number), turn| match memory.insert(last_number, turn - 1) {
            Some(previous_turn) => {
                let age = turn - previous_turn - 1;
                // println!( "memory: {:?}", memory );
                // println!( "turn: {} - last number was {} (last spoken at {}).  Number (age) for this turn: {}", turn, last_number, previous_turn, age );
                (memory, age)
            }
            None => {
                // println!( "memory: {:?}", memory );
                // println!( "turn: {} - last number was {} (never spoken).  Number for this turn: 0", turn, last_number );
                (memory, 0)
            }
        },
    );
    last_number
}
