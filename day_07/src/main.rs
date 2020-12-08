extern crate im_rc;
#[macro_use]
extern crate lazy_static;
extern crate pbr;
extern crate regex;

use im_rc::Vector;
use pbr::ProgressBar;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::FromIterator;

fn main() {
    println!("--- [AoC 2020] Day 7: Handy Haversacks ---");
    let input: Vector<String> = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let bags = parse_bags(input);
    let mut progress = ProgressBar::new(bags.len().try_into().unwrap());

    let count = bags
        .iter()
        .filter(|(_, bag)| {
            let b = HashMap::from_iter(bags.iter());
            let result = bag.can_contain(&"shiny gold", &b);
            progress.inc();
            result
        })
        .count();
    progress.finish();
    println!("\nSolution to part one: {}", count);
}

fn part_two(input: &Vector<String>) {
    let bags = parse_bags(input);
    let shiny_golden = bags.get("shiny gold").unwrap();
    let result = shiny_golden.count_bags(&bags) - 1;
    println!("Solution to part two: {}", result);
}

fn parse_bags(input: &Vector<String>) -> HashMap<String, Bag> {
    input.iter().fold(HashMap::new(), |mut bags, line| {
        let bag = Bag::parse(line);
        let color = bag.color.to_owned();
        bags.insert(color, bag);
        bags
    })
}

#[derive(PartialEq, Debug, Clone)]
struct Bag {
    color: String,
    contents: HashMap<String, usize>,
}

impl Bag {
    fn count_bags(&self, bags: &HashMap<String, Bag>) -> usize {
        self.contents.iter().fold(1, |count, (color, number)| {
            count + number * bags.get(color).unwrap().count_bags(bags)
        })
    }

    fn can_contain(&self, color: &str, bags: &HashMap<&String, &Bag>) -> bool {
        self.can_directly_contain(color) || {
            let left_over_bags: HashMap<&String, &Bag> = bags
                .iter()
                .filter(|(&c, _)| c != &self.color)
                .map(|(&c, &b)| (c, b))
                .collect();
            self.can_recursively_contain(color, &left_over_bags)
        }
    }

    fn can_directly_contain(&self, color: &str) -> bool {
        self.contents.get(color).unwrap_or(&0) > &0
    }

    fn can_recursively_contain(&self, color: &str, bags: &HashMap<&String, &Bag>) -> bool {
        self.contents.iter().any(|(contained_color, count)| {
            count > &0 && {
                bags.get(contained_color)
                    .map(|bag| bag.can_contain(color, bags))
                    .unwrap_or(false)
            }
        })
    }

    fn parse(input: &str) -> Bag {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<color>[a-z ]+) bags contain (?P<contents>.+).$").unwrap();
        }

        let captures = RE.captures(input).unwrap();
        let color = captures.name("color").unwrap().as_str();

        match captures.name("contents") {
            Some(empty) if empty.as_str() == "no other bags" => Bag::empty(color),
            Some(contents_string) => {
                let contents = contents_string.as_str().split(", ").fold(
                    HashMap::new(),
                    |mut contents, line| {
                        lazy_static! {
                            static ref CONTENTS_RE: Regex =
                                Regex::new("^(?P<count>[0-9]+) (?P<color>[a-z ]+) bags?$").unwrap();
                        }

                        let captures = CONTENTS_RE.captures(line).unwrap();
                        let count = captures
                            .name("count")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();
                        let color = captures.name("color").unwrap().as_str();
                        contents.insert(color.to_owned(), count);
                        contents
                    },
                );
                Bag::new(color, contents)
            }
            _ => panic!("Could not parse line"),
        }
    }

    fn new(color: &str, contents: HashMap<String, usize>) -> Bag {
        Bag {
            color: color.to_owned(),
            contents,
        }
    }

    fn empty(color: &str) -> Bag {
        Bag {
            color: color.to_owned(),
            contents: HashMap::new(),
        }
    }
}
