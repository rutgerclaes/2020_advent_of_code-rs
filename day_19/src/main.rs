extern crate im_rc;

use im_rc::hashset;
use im_rc::vector;
use im_rc::HashMap;
use im_rc::HashSet;
use im_rc::Vector;
use itertools::Itertools;

fn main() {
    println!("--- [AoC 2020] Day 19: Monster Messages ---");

    let input = utils::read_strings_from_param();

    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> usize {
    let rules = Rule::parse_all(input.iter().take_while(|line| !line.is_empty()).collect());
    let rule_zero = rules.get(&0).unwrap();
    input
        .iter()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .filter(|line| {
            rule_zero
                .matches(line, &rules)
                .map(|res| res.iter().any(|s| s.is_empty()))
                .unwrap_or(false)
        })
        .count()
}

fn part_two(input: &Vector<String>) -> usize {
    let rules = Rule::parse_all(input.iter().take_while(|line| !line.is_empty()).collect());

    let part_two_changes = vector!("8: 42 | 42 8", "11: 42 31 | 42 11 31");
    let rules = part_two_changes.iter().fold(rules, |rs, line| {
        let (nb, r) = Rule::parse(line);
        rs.update(nb, r)
    });
    let rule_zero = rules.get(&0).unwrap();
    input
        .iter()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .filter(|line| {
            rule_zero
                .matches(line, &rules)
                .map(|res| res.iter().any(|s| s.is_empty()))
                .unwrap_or(false)
        })
        .count()
}

#[derive(Debug, Clone)]
enum Rule {
    Character(char),
    Sequence(Vector<i32>),
    Either(Vector<Rule>),
}

impl Rule {
    fn matches<'a>(
        &self,
        pattern: &'a str,
        rules: &HashMap<i32, Rule>,
    ) -> Result<HashSet<&'a str>, ()> {
        match self {
            Self::Character(ch) if pattern.chars().next().map(|c| &c == ch).unwrap_or(false) => {
                Ok(hashset!(&pattern[1..]))
            }
            Self::Character(_) => Err(()),
            Self::Either(options) => {
                let next_tails = options
                    .iter()
                    .filter_map(|option| match option.matches(pattern, rules) {
                        Ok(tails) => Some(tails),
                        Err(()) => None,
                    })
                    .fold(HashSet::new(), |a, b| a.union(b));
                if next_tails.is_empty() {
                    Err(())
                } else {
                    Ok(next_tails)
                }
            }
            Self::Sequence(sequence) => {
                let seq_rules: Vector<_> =
                    sequence.iter().map(|nb| rules.get(nb).unwrap()).collect();
                let outcomes = seq_rules
                    .iter()
                    .try_fold(hashset!(pattern), |inputs, rule| {
                        let outputs: HashSet<_> = inputs
                            .iter()
                            .filter_map(|input| match rule.matches(input, rules) {
                                Ok(outs) => Some(outs),
                                Err(_) => None,
                            })
                            .flatten()
                            .collect();

                        if outputs.is_empty() {
                            Err(())
                        } else {
                            Ok(outputs)
                        }
                    });

                match outcomes {
                    Err(()) => Err(()),
                    Ok(res) if res.is_empty() => Err(()),
                    Ok(res) => Ok(res),
                }
            }
        }
    }

    fn parse_all(input: Vector<&String>) -> HashMap<i32, Rule> {
        input.iter().map(|line| Self::parse(&line[..])).collect()
    }

    fn parse(input: &str) -> (i32, Rule) {
        let (number_part, rule_part) = input.split(':').collect_tuple().unwrap();
        let number = number_part.parse::<i32>().unwrap();
        let rule = Self::parse_rule(rule_part.trim());
        (number, rule)
    }

    fn parse_rule(input: &str) -> Rule {
        let pieces: Vector<_> = input.split('|').collect();
        if pieces.len() == 1 {
            let piece = pieces.head().unwrap();
            println!("{}", piece);
            match piece.chars().next().unwrap() {
                ' ' => Self::parse_rule(&piece[1..]),
                '"' => Rule::Character(piece.chars().nth(1).unwrap()),
                ch if ch.is_ascii_digit() => Rule::Sequence(
                    piece
                        .trim()
                        .split(' ')
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect(),
                ),
                _ => panic!("Unsupported rule {}", input),
            }
        } else {
            Rule::Either(pieces.iter().map(|piece| Self::parse_rule(piece)).collect())
        }
    }
}
