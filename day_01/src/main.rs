extern crate im_rc;

use im_rc::Vector;
use itertools::Itertools;
use utils::read_integers_from_param;

fn main() {
    println!("--- [AoC 2020] Day 1: Report Repair ---");
    let input: Vector<i64> = read_integers_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<i64>) {
    match find_complement(2020, input) {
        Some((a, b)) => println!("Solution for part one is: {}", a * b),
        None => panic!("No results found for part one"),
    }
}

fn part_two(input: &Vector<i64>) {
    match input
        .iter()
        .combinations(3)
        .find(|triplet| triplet[0] + triplet[1] + triplet[2] == 2020)
    {
        Some(triplet) => println!(
            "Solution for part two is: {}",
            triplet[0] * triplet[1] * triplet[2]
        ),
        None => panic!("No results found for part two"),
    }
}

fn find_complement(total: i64, other_candidates: &Vector<i64>) -> Option<(i64, i64)> {
    match other_candidates.head() {
        Some(&head) => match find_complement_of(total, head, &other_candidates.skip(1)) {
            Some(complement) => Some((head, complement)),
            None => find_complement(total, &other_candidates.skip(1)),
        },
        None => None,
    }
}

fn find_complement_of(total: i64, candidate: i64, other_candidates: &Vector<i64>) -> Option<i64> {
    match other_candidates.head() {
        Some(&head) if total == candidate + head => Some(head),
        Some(_head) => find_complement_of(total, candidate, &other_candidates.skip(1)),
        None => None,
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use im_rc::vector;

    #[test]
    fn test_find_complement_of() {
        let input = vector!(1, 2, 3, 4);
        assert_eq!(Some(4), find_complement_of(5, 1, &input));
        assert_eq!(Some(2), find_complement_of(5, 3, &input));
        assert_eq!(None, find_complement_of(6, 1, &input));
    }

    #[test]
    fn test_find_complement() {
        let input = vector!(2, 1, 5, 3);
        assert_eq!(Some((2, 3)), find_complement(5, &input));
        assert_eq!(Some((1, 5)), find_complement(6, &input));
        assert_eq!(None, find_complement(9, &input));
    }
}
