extern crate im_rc;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use im_rc::vector;
use im_rc::Vector;
use pbr::ProgressBar;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;

fn main() {
    println!("--- [AoC 2020] Day 14: Docking Data ---");

    let input = utils::read_strings_from_param();
    println!("Solution to part one: {}", part_one(&input).unwrap());
    println!("Solution to part two: {}", part_two(&input).unwrap());
}

fn part_one(input: &Vector<String>) -> Option<u64> {
    let mut progress = ProgressBar::new(input.len() as u64);
    let (memory, _) =
        input.iter().fold(
            (HashMap::new(), None),
            |(mut memory, mask), line| match Mask::try_from(&line[..]) {
                Ok(new_mask) => {
                    progress.inc();
                    (memory, Some(new_mask))
                }
                _ => {
                    let instruction = Instruction::try_from(&line[..]).unwrap();
                    memory.insert(
                        instruction.address,
                        mask.map(|m| m.translate(instruction.value))
                            .unwrap_or(instruction.value),
                    );
                    progress.inc();
                    (memory, mask)
                }
            },
        );
    progress.finish();
    Some(memory.values().sum())
}

fn part_two(input: &Vector<String>) -> Option<u64> {
    let mut progress = ProgressBar::new(input.len() as u64);
    let (memory, _) =
        input.iter().fold(
            (HashMap::new(), None),
            |(mut memory, mask), line| match Mask::try_from(&line[..]) {
                Ok(new_mask) => {
                    progress.inc();
                    (memory, Some(new_mask))
                }
                _ => {
                    let instruction = Instruction::try_from(&line[..]).unwrap();
                    match mask {
                        Some(m) => {
                            let addresses: Vector<_> = m.map_address(instruction.address);
                            for address in addresses {
                                memory.insert(address, instruction.value);
                            }
                            progress.inc();
                        }
                        None => {
                            memory.insert(instruction.address, instruction.value);
                            progress.inc();
                        }
                    }
                    (memory, mask)
                }
            },
        );

    progress.finish();
    Some(memory.values().sum())
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction {
    address: u64,
    value: u64,
}

impl Instruction {
    fn new(address: u64, value: u64) -> Instruction {
        Instruction { address, value }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mem[{}] = {}", self.address, self.value)
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(line: &str) -> Result<Instruction, Self::Error> {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"^mem\[(?P<address>[0-9]+)\] = (?P<value>[0-9]+)$").unwrap();
        }

        INSTRUCTION_RE
            .captures(line)
            .map(|capture| {
                let address = capture
                    .name("address")
                    .map(|val| val.as_str().parse::<u64>().unwrap())
                    .unwrap();
                let value = capture
                    .name("value")
                    .map(|val| val.as_str().parse::<u64>().unwrap())
                    .unwrap();
                Instruction::new(address, value)
            })
            .ok_or(())
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Mask {
    positive: u64,
    negative: u64,
}

impl Mask {
    fn new(value: &str) -> Mask {
        let (pos, neg) = value.chars().fold((0, 0), |(pos, neg), c| match c {
            '0' => (pos << 1, (neg << 1) + 1),
            '1' => ((pos << 1) + 1, neg << 1),
            _ => (pos << 1, neg << 1),
        });

        Mask {
            positive: pos,
            negative: neg,
        }
    }

    fn translate(&self, value: u64) -> u64 {
        (value | self.positive) & !self.negative
    }

    fn map_address(&self, address: u64) -> Vector<u64> {
        let floating = !(self.positive | self.negative);
        let with_overrides = address | self.positive;

        (0..36).filter(|i| floating & 1 << i > 0).fold(
            vector!(with_overrides),
            |res, floating_index| {
                res.iter()
                    .flat_map(|number| vector!(*number, number ^ 1 << floating_index))
                    .collect()
            },
        )
    }
}

impl std::fmt::Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let positive_debug = format!("{:036b}", self.positive);
        let negative_debug = format!("{:036b}", self.negative);
        let chars = positive_debug
            .chars()
            .zip(negative_debug.chars())
            .map(|(pos, neg)| {
                if neg == '1' {
                    '0'
                } else if pos == '1' {
                    '1'
                } else {
                    'X'
                }
            });
        let output = String::from_iter(chars);
        write!(f, "mask = {}", output)
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mask( pos: {:036b} neg: {:036b})",
            self.positive, self.negative
        )
    }
}

impl TryFrom<&str> for Mask {
    type Error = ();

    fn try_from(line: &str) -> Result<Mask, Self::Error> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = (?P<mask>[01X]+)$").unwrap();
        }
        MASK_RE
            .captures(line)
            .and_then(|res| res.name("mask").map(|r| r.as_str()))
            .map(|value| Mask::new(value))
            .ok_or(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_mask_buildling() {
        let mask = Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.positive, 64);
        assert_eq!(mask.negative, 2);
    }

    #[test]
    fn test_mask_parsing() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask = Mask::try_from(input);
        assert_eq!(mask, Ok(Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")));
    }

    #[test]
    fn test_mask_display() {
        let input = "mask = 1XXXXXXXXXXXXXX0XXXXXXXXXXXXX1XXXX0X";
        let mask = Mask::try_from(input);
        let display = format!("{}", mask.unwrap());
        assert_eq!(display, input);
    }

    #[test]
    fn test_instruction_parsing() {
        let input = "mem[8] = 11";
        let oper = Instruction::try_from(input);
        assert_eq!(
            oper,
            Ok(Instruction {
                address: 8,
                value: 11
            })
        );
    }

    #[test]
    fn test_instruction_display() {
        let input = "mem[8] = 11";
        let oper = Instruction::try_from(input).unwrap();
        let display = format!("{}", oper);
        assert_eq!(display, input);
    }

    #[test]
    fn test_mask_memory_mapping() {
        let mask = Mask::new("000000000000000000000000000000X1001X");
        let mut results = mask.map_address(42);
        results.sort();
        assert_eq!(results, vector!(26, 27, 58, 59));
    }
}
