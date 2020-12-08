extern crate im_rc;
extern crate itertools;

use im_rc::Vector;
use itertools::Itertools;

fn main() {
    println!("--- [AoC 2020] Day 5: Binary Boarding ---");
    let input: Vector<String> = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let result = input
        .iter()
        .map(|line| {
            let (row, seat) = parse_seat(line);
            row * 8 + seat
        })
        .max()
        .unwrap();
    println!("Solution to part one: {}", result);
}

fn part_two(input: &Vector<String>) {
    let mut seat_ids: Vector<usize> = input
        .iter()
        .map(|line| {
            let (row, seat) = parse_seat(line);
            row * 8 + seat
        })
        .collect();

    seat_ids.sort();

    let result = seat_ids
        .iter()
        .tuple_windows()
        .find(|(&a, &b)| a != b - 1)
        .map(|(a, _)| a + 1)
        .unwrap();

    println!("Solution to part two: {}", result);
}

fn parse_seat(input: &str) -> (usize, usize) {
    let ((min_row, max_row), (min_seat, max_seat)) = input.chars().fold(
        ((0, 127), (0, 7)),
        |((min_row, max_row), (min_seat, max_seat)), next_char| match next_char {
            'F' => (
                (min_row, min_row + (max_row - min_row) / 2),
                (min_seat, max_seat),
            ),
            'B' => (
                (1 + min_row + (max_row - min_row) / 2, max_row),
                (min_seat, max_seat),
            ),
            'L' => (
                (min_row, max_row),
                (min_seat, min_seat + (max_seat - min_seat) / 2),
            ),
            'R' => (
                (min_row, max_row),
                (1 + min_seat + (max_seat - min_seat) / 2, max_seat),
            ),
            _ => panic!("Unforseen char encountered"),
        },
    );

    if min_row != max_row {
        panic!("Min row does not match with max row")
    }
    if min_seat != max_seat {
        panic!("Min seat does not match with max row")
    }

    (min_row, min_seat)
}
