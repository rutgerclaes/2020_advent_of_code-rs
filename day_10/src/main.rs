extern crate im_rc;
extern crate itertools;

use im_rc::Vector;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("--- [AoC 2020] Day 10: Adapter Array ---");
    let input = utils::read_integers_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<i64>) {
    let mut sorted_input: Vector<_> = input.iter().collect();
    sorted_input.sort();
    sorted_input.push_front(&0);
    let (delta_1, delta_3) = sorted_input
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| b - a)
        .fold((0, 1), |(delta_1, delta_3), delta| match delta {
            1 => (delta_1 + 1, delta_3),
            3 => (delta_1, delta_3 + 1),
            _ => panic!("Found unknown delta"),
        });
    let result = delta_1 * delta_3;
    println!(
        "Solution to part one: {} ({} * {})",
        result, delta_1, delta_3
    );
}

fn part_two(input: &Vector<i64>) {
    let mut sorted_input: Vector<_> = input.iter().collect();
    sorted_input.sort();
    sorted_input.push_front(&0); // Add charging outlet

    // ways is a cache that contains the number of paths (combinations) that lead to
    // a given adapter.
    let mut ways: HashMap<i64, usize> = HashMap::new();
    ways.insert(0, 1); // There is exactly one path to start: the charging outlet.

    for pos in 1..sorted_input.len() {
        let element = sorted_input.get(pos).copied().unwrap();

        // Calculate the elements via which you could reach `element`
        let prior_elements = (0..3).filter_map(|offset| {
            sorted_input
                .get(pos - offset - 1)
                .filter(|&prior_element| *prior_element >= &(element - 3))
        });

        // Get the number of ways you could reach each of the possible
        // prior elements.  We've calculated each of them in previous
        // iterations.
        // The number of paths leading to `element` is the sum of the number of paths
        // you can take to each of the individual prior elements.
        let paths_via_element = prior_elements
            .map(|prior_elem| ways.get(prior_elem).unwrap())
            .sum();

        // Store the number of paths leading to `element`
        ways.insert(*element, paths_via_element);
        // Clean up the number of paths to elements that are out of scope (too far behind).
        ways.retain(|&k, _| k > element - 3);
    }

    let result = ways.get(sorted_input.last().unwrap()).unwrap();

    println!("Solution to part two: {}", result);
}
