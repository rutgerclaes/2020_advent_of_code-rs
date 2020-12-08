#[macro_use]
extern crate lazy_static;
extern crate im_rc;
extern crate regex;

use im_rc::*;
use regex::Regex;
use std::convert::TryFrom;
use std::convert::TryInto;

fn main() {
    println!("--- [AoC 2020] Day 2: Password Philosophy ---");
    let input: Vector<String> = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    character: char,
    min_appearances: i32,
    max_appearances: i32,
}

impl PasswordPolicy {
    fn complies(&self, input: &str) -> bool {
        let password = input
            .split_terminator(':')
            .last()
            .expect(&format!("No password field in input '{}'", input));
        let character_count = password.chars().filter(|c| *c == self.character).count();
        (character_count >= self.min_appearances.try_into().unwrap())
            && (character_count <= self.max_appearances.try_into().unwrap())
    }

    fn complies_v2(&self, input: &str) -> bool {
        let password = input
            .split_terminator(": ")
            .last()
            .expect(&format!("No password field in input '{}'", input));
        let chars: Vec<char> = password.chars().collect();
        let first_position: usize = self.min_appearances.try_into().unwrap();
        let first_character = chars[first_position - 1];

        let last_position: usize = self.max_appearances.try_into().unwrap();
        let last_character = chars[last_position - 1];
        let one_matches = first_character == self.character || last_character == self.character;
        let two_matches = first_character == self.character && last_character == self.character;

        one_matches && !two_matches
    }
}

impl TryFrom<&str> for PasswordPolicy {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<char>[a-z])(:.*)?$").unwrap();
        }

        match RE.captures(input) {
            Some(captures) => Ok(PasswordPolicy {
                character: captures
                    .name("char")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap(),
                min_appearances: captures
                    .name("min")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                max_appearances: captures
                    .name("max")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
            }),
            None => Err(()),
        }
    }
}

fn part_one(input: &Vector<String>) {
    let nb_of_valid_passwords = input.iter().filter(|line| password_is_valid(&line)).count();
    println!("Solution for part one: {}", nb_of_valid_passwords);
}

fn part_two(input: &Vector<String>) {
    let nb_of_valid_passwords = input
        .iter()
        .filter(|line| password_is_valid_v2(&line))
        .count();
    println!("Solution for part two: {}", nb_of_valid_passwords);
}

fn password_is_valid(input: &str) -> bool {
    let policy =
        PasswordPolicy::try_from(input).expect(&format!("Could not parse policy in {}", input));
    policy.complies(input)
}

fn password_is_valid_v2(input: &str) -> bool {
    let policy =
        PasswordPolicy::try_from(input).expect(&format!("Could not parse policy in {}", input));
    policy.complies_v2(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_password_policy_parsing() {
        let input = "1-3 a: abcde";
        let result = PasswordPolicy::try_from(input);
        let expected = PasswordPolicy {
            character: 'a',
            min_appearances: 1,
            max_appearances: 3,
        };
        assert_eq!(result.unwrap(), expected);

        let input = "1-3 b: cdefg";
        let result = PasswordPolicy::try_from(input);
        let expected = PasswordPolicy {
            character: 'b',
            min_appearances: 1,
            max_appearances: 3,
        };
        assert_eq!(result.unwrap(), expected);

        let input = "2-9 c: ccccccccc";
        let result = PasswordPolicy::try_from(input);
        let expected = PasswordPolicy {
            character: 'c',
            min_appearances: 2,
            max_appearances: 9,
        };
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_password_compliance() {
        let input = "1-3 a: abcde";
        let policy = PasswordPolicy::try_from(input).unwrap();
        assert!(policy.complies(input));

        let input = "1-3 b: cdefg";
        let policy = PasswordPolicy::try_from(input).unwrap();
        assert!(!policy.complies(input));

        let input = "2-9 c: ccccccccc";
        let policy = PasswordPolicy::try_from(input).unwrap();
        assert!(policy.complies(input));
    }

    #[test]
    fn test_password_compliance_v2() {
        let input = "1-3 a: abcde";
        let policy = PasswordPolicy::try_from(input).unwrap();
        assert!(policy.complies_v2(input));

        let input = "1-3 b: cdefg";
        let policy = PasswordPolicy::try_from(input).unwrap();
        assert!(!policy.complies_v2(input));

        let input = "2-9 c: ccccccccc";
        let policy = PasswordPolicy::try_from(input).unwrap();
        assert!(!policy.complies_v2(input));
    }
}
