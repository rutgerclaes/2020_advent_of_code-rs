extern crate im_rc;
extern crate itertools;

use im_rc::Vector;
use itertools::Itertools;
use std::cmp;

fn main() {
    println!("--- [AoC 2020] Day 9: Encoding Error ---");
    let input = utils::read_integers_from_param();
    let preamble_size = 25;
    let solution_part_one = part_one(&input, preamble_size);
    part_two(&input, solution_part_one);
}

fn part_one(input: &Vector<i64>, preamble_size: usize) -> i64 {
    let result = input
        .iter()
        .collect::<Vec<_>>()
        .windows(preamble_size + 1)
        .find_map(|window| {
            let sum = window[window.len() - 1];
            let terms: Vec<_> = window[0..window.len() - 1].iter().copied().collect();

            let is_sum = terms
                .iter()
                .tuple_combinations()
                .any(|(&a, &b)| a + b == *sum);

            Some(sum).filter(|_| !is_sum)
        })
        .unwrap();

    println!("Solution to part one: {}", result);
    *result
}

fn part_two(input: &Vector<i64>, search_sum: i64) {
    let result = (0..input.len())
        .find_map(|offset| {
            let result =
                input
                    .iter()
                    .skip(offset)
                    .try_fold((0, None, None), |(sum, min, max), elem| match sum + elem {
                        new_sum if new_sum == search_sum => {
                            let result = min.map(|v| cmp::min(v, elem)).unwrap_or(elem)
                                + max.map(|v| cmp::max(v, elem)).unwrap_or(elem);
                            Err(Some(result))
                        }
                        new_sum if new_sum < search_sum => Ok((
                            new_sum,
                            Some(min.map(|v| cmp::min(v, elem)).unwrap_or(elem)),
                            Some(max.map(|v| cmp::max(v, elem)).unwrap_or(elem)),
                        )),
                        _ => Err(None),
                    });

            match result {
                Err(Some(value)) => Some(value),
                _ => None,
            }
        })
        .unwrap();

    println!("Solution to part two: {}", result);
}
