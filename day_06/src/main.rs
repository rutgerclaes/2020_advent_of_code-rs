extern crate im_rc;

use im_rc::Vector;
use std::collections::HashSet;

fn main() {
    println!("--- [AoC 2020] Day 6: Custom Customs ---");
    let input: Vector<String> = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let (count, last) = input
        .iter()
        .fold((0, HashSet::new()), |(count, answers), line| {
            if line.is_empty() {
                (count + answers.len(), HashSet::new())
            } else {
                let updated_answers = line.chars().fold(answers, |mut set, c| {
                    set.insert(c);
                    set
                });
                (count, updated_answers)
            }
        });

    let result = count + last.len();
    println!("The solution to part one: {}", result);
}

fn part_two(input: &Vector<String>) {
    let (count, last_chars, last_lines) = input.iter().fold(
        (0, HashSet::new(), Vector::new()),
        |(count, chars, mut lines): (usize, HashSet<char>, Vector<&str>), line| {
            if line.is_empty() {
                let addition = chars
                    .iter()
                    .filter(|&ch| lines.iter().all(|line| line.contains(*ch)))
                    .count();
                (count + addition, HashSet::new(), Vector::new())
            } else {
                let updated_chars = line.chars().fold(chars, |mut set, c| {
                    set.insert(c);
                    set
                });
                lines.push_back(line);
                (count, updated_chars, lines)
            }
        },
    );

    let result = count
        + last_chars
            .iter()
            .filter(|&c| last_lines.iter().all(|&line| line.contains(*c)))
            .count();

    println!("The solution to part two: {}", result);
}
