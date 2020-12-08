extern crate im_rc;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use im_rc::vector;
use im_rc::Vector;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("--- [AoC 2020] Day 4: Passport Processing ---");
    let input: Vector<String> = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let documents = PasswordData::parse(input);
    let nb_of_valid_documents = documents
        .iter()
        .filter(|d| d.contains_all_necessary_fields())
        .count();
    println!("Solution to part one: {}", nb_of_valid_documents);
}

fn part_two(input: &Vector<String>) {
    let documents = PasswordData::parse(input);
    let nb_of_valid_documents = documents.iter().filter(|d| d.is_valid()).count();
    println!("Solution to part two: {}", nb_of_valid_documents);
}

#[derive(Debug, Clone, PartialEq)]
struct PasswordData {
    fields: HashMap<String, String>,
}

impl PasswordData {
    fn contains_all_necessary_fields(&self) -> bool {
        let required_fields = vector!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid");
        required_fields
            .iter()
            .all(|field| self.contains_field(field))
    }

    fn check_numeric_field(&self, field_name: &str, lower_limit: usize, upper_limt: usize) -> bool {
        self.fields
            .get(field_name)
            .map(|value| {
                value
                    .parse::<usize>()
                    .map(|i| i >= lower_limit && i <= upper_limt)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    fn byr_field_is_valid(&self) -> bool {
        self.check_numeric_field("byr", 1920, 2002)
    }

    fn iyr_field_is_valid(&self) -> bool {
        self.check_numeric_field("iyr", 2010, 2020)
    }

    fn eyr_field_is_valid(&self) -> bool {
        self.check_numeric_field("eyr", 2020, 2030)
    }

    fn hgt_field_is_valid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<value>[0-9]+)(?P<unit>cm|in)").unwrap();
        }

        self.fields
            .get("hgt")
            .map(|value| match RE.captures(value) {
                Some(captures)
                    if captures
                        .name("unit")
                        .map(|u| u.as_str() == "cm")
                        .unwrap_or(false) =>
                {
                    let height = captures
                        .name("value")
                        .map(|mat| mat.as_str())
                        .unwrap_or("0")
                        .parse::<usize>()
                        .unwrap_or(0);
                    height >= 150 && height <= 193
                }
                Some(captures)
                    if captures
                        .name("unit")
                        .map(|u| u.as_str() == "in")
                        .unwrap_or(false) =>
                {
                    let height = captures
                        .name("value")
                        .map(|mat| mat.as_str())
                        .unwrap_or("0")
                        .parse::<usize>()
                        .unwrap_or(0);
                    height >= 59 && height <= 76
                }
                _ => false,
            })
            .unwrap_or(false)
    }

    fn check_regex_field(&self, field: &str, re: &Regex) -> bool {
        self.fields
            .get(field)
            .map(|value| re.is_match(value))
            .unwrap_or(false)
    }

    fn hcl_field_is_valid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        self.check_regex_field("hcl", &RE)
    }

    fn ecl_field_is_valid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }

        self.check_regex_field("ecl", &RE)
    }

    fn pid_field_is_valid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        self.check_regex_field("pid", &RE)
    }

    fn is_valid(&self) -> bool {
        self.byr_field_is_valid()
            && self.iyr_field_is_valid()
            && self.eyr_field_is_valid()
            && self.hgt_field_is_valid()
            && self.hcl_field_is_valid()
            && self.ecl_field_is_valid()
            && self.pid_field_is_valid()
    }

    fn add_field(mut self, key: &str, value: &str) -> PasswordData {
        self.fields.insert(key.to_owned(), value.to_owned());
        self
    }

    fn contains_field(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    fn extract_fields(input: &str) -> HashMap<String, String> {
        input
            .split_whitespace()
            .map(|data| data.split_terminator(':').collect::<Vec<_>>())
            .fold(HashMap::new(), |mut fields, pieces| {
                fields.insert(pieces[0].to_owned(), pieces[1].to_owned());
                fields
            })
    }

    fn add_fields(self, fields: &HashMap<String, String>) -> PasswordData {
        fields.iter().fold(self, |password_data, (key, value)| {
            password_data.add_field(key, value)
        })
    }

    fn new(fields: HashMap<String, String>) -> PasswordData {
        PasswordData { fields }
    }

    fn parse(input: &Vector<String>) -> Vector<PasswordData> {
        input
            .iter()
            .map(|line| PasswordData::extract_fields(line))
            .fold(Vector::<PasswordData>::new(), |mut documents, fields| {
                if fields.is_empty() || documents.is_empty() {
                    documents.push_back(PasswordData::new(fields));
                    documents
                } else {
                    let last_document = documents.pop_back().unwrap();
                    documents.push_back(last_document.add_fields(&fields));
                    documents
                }
            })
    }
}
