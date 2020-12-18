extern crate im_rc;

use im_rc::Vector;
use std::convert;
use std::convert::TryFrom;
use std::fmt;

fn main() {
    println!("--- [AoC 2020] Day 18: Operation Order ---");

    let input = utils::read_strings_from_param();

    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> i64 {
    input
        .iter()
        .map(|line| Expression::parse(line).unwrap().value())
        .sum()
}

fn part_two(input: &Vector<String>) -> i64 {
    input
        .iter()
        .map(|line| Expression::parse_adv(line).unwrap().value())
        .sum()
}

#[derive(Debug)]
enum Operand {
    Multiplication,
    Addition,
}

impl Operand {
    fn apply(&self, a: &Expression, b: &Expression) -> i64 {
        match self {
            Self::Multiplication => a.value() * b.value(),
            Self::Addition => a.value() + b.value(),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Multiplication => write!(f, "*"),
            Self::Addition => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
enum Expression {
    Number(i64),
    Calculation(Box<Expression>, Operand, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(i) => write!(f, "{}", i),
            Self::Calculation(left, operand, right) => {
                write!(f, "({} {} {})", left, operand, right)
            }
        }
    }
}

impl convert::From<Expression> for i64 {
    fn from(input: Expression) -> i64 {
        input.value()
    }
}

#[derive(Debug)]
enum ParseError {
    Generic(String),
    ParenthesisParsing(String),
    OperatorParsing(String),
    NumberParsing(String),
}

impl Expression {
    fn new_calculation(a: Self, operand: Operand, b: Self) -> Self {
        Self::Calculation(Box::new(a), operand, Box::new(b))
    }

    fn value(&self) -> i64 {
        match self {
            Self::Number(v) => *v,
            Self::Calculation(a, operand, b) => operand.apply(a, b),
        }
    }

    fn parse(input: &str) -> Result<Self, ParseError> {
        if let Some((first, remainder)) = Self::take_next(input)? {
            first.combine_with_remainder(remainder)
        } else {
            Ok(Self::Number(0))
        }
    }

    fn parse_adv(input: &str) -> Result<Self, ParseError> {
        if let Some((first, remainder)) = Self::take_next_adv(input)? {
            first.combine_with_remainder_adv(remainder)
        } else {
            Ok(Self::Number(0))
        }
    }

    fn combine_with_remainder(self, input: &str) -> Result<Self, ParseError> {
        match Self::take_operand(input)? {
            Some((operand, remainder)) => match Self::take_next(remainder)? {
                Some((next, next_input)) => {
                    let next_combo = Self::new_calculation(self, operand, next);
                    next_combo.combine_with_remainder(next_input)
                }
                None => Err(ParseError::Generic(format!(
                    "Failed to get next term from '{}'",
                    remainder
                ))),
            },
            None => Ok(self),
        }
    }

    fn combine_with_remainder_adv(self, input: &str) -> Result<Self, ParseError> {
        match Self::take_operand(input)? {
            Some((Operand::Addition, remainder)) => match Self::take_next_adv(remainder)? {
                Some((next, next_input)) => {
                    let next_combo = Self::new_calculation(self, Operand::Addition, next);
                    next_combo.combine_with_remainder_adv(next_input)
                }
                None => Err(ParseError::Generic(format!(
                    "Failed to get next term from '{}'",
                    remainder
                ))),
            },
            Some((Operand::Multiplication, remainder)) => match Self::take_next_adv(remainder)? {
                Some((next, next_input)) => {
                    let tail = next.combine_with_remainder_adv(next_input)?;
                    Ok(Self::new_calculation(self, Operand::Multiplication, tail))
                }
                None => Err(ParseError::Generic(format!(
                    "Failed to get next term from '{}'",
                    remainder
                ))),
            },
            None => Ok(self),
        }
    }

    fn take_operand(input: &str) -> Result<Option<(Operand, &str)>, ParseError> {
        match input.chars().next() {
            Some(' ') => Self::take_operand(&input[1..]),
            Some('+') => Ok(Some((Operand::Addition, &input[2..]))),
            Some('*') => Ok(Some((Operand::Multiplication, &input[2..]))),
            Some(_) => Err(ParseError::OperatorParsing(format!(
                "Failed to parse {}",
                input
            ))),
            None => Ok(None),
        }
    }

    fn take_next(input: &str) -> Result<Option<(Self, &str)>, ParseError> {
        match input.chars().next() {
            None => Ok(None),
            Some(' ') => Self::take_next(&input[1..]),
            Some(c) if c.is_ascii_digit() => Self::take_number(input).map(Some),
            Some('(') => Self::take_parenthesis(input).map(Some),
            _ => Err(ParseError::Generic(format!("Error parsing '{}'", input))),
        }
    }

    fn take_next_adv(input: &str) -> Result<Option<(Self, &str)>, ParseError> {
        match input.chars().next() {
            None => Ok(None),
            Some(' ') => Self::take_next(&input[1..]),
            Some(c) if c.is_ascii_digit() => Self::take_number(input).map(Some),
            Some('(') => Self::take_parenthesis_adv(input).map(Some),
            _ => Err(ParseError::Generic(format!("Error parsing '{}'", input))),
        }
    }

    fn take_parenthesis(expression: &str) -> Result<(Self, &str), ParseError> {
        let length = expression
            .chars()
            .try_fold((-1, 0), |(level, length), ch| match ch {
                ')' if level == 0 => Err(length),
                ')' => Ok((level - 1, length + 1)),
                '(' => Ok((level + 1, length + 1)),
                _ => Ok((level, length + 1)),
            });

        match length {
            Err(length) => {
                let first = Self::parse(&expression[1..length])?;
                let tail = &expression[length + 1..];
                Ok((first, tail))
            }
            Ok(_) => Err(ParseError::ParenthesisParsing(format!(
                "Malformed input '{}'",
                expression
            ))),
        }
    }

    fn take_parenthesis_adv(expression: &str) -> Result<(Self, &str), ParseError> {
        let length = expression
            .chars()
            .try_fold((-1, 0), |(level, length), ch| match ch {
                ')' if level == 0 => Err(length),
                ')' => Ok((level - 1, length + 1)),
                '(' => Ok((level + 1, length + 1)),
                _ => Ok((level, length + 1)),
            });

        match length {
            Err(length) => {
                let first = Self::parse_adv(&expression[1..length])?;
                let tail = &expression[length + 1..];
                Ok((first, tail))
            }
            Ok(_) => Err(ParseError::ParenthesisParsing(format!(
                "Malformed input '{}'",
                expression
            ))),
        }
    }

    fn take_number(expression: &str) -> Result<(Self, &str), ParseError> {
        let length = expression.chars().try_fold(0, |length, ch| match ch {
            ch if ch.is_ascii_digit() => Ok(length + 1),
            _ => Err(length),
        });

        match length {
            Err(length) | Ok(length) if length > 0 => {
                let value = &expression[0..length].parse::<i64>().unwrap(); // TODO deal with error

                let remainder = if expression.len() > length {
                    &expression[(length + 1)..]
                } else {
                    ""
                };

                Ok((Self::Number(*value), remainder))
            }
            Err(_) | Ok(_) => Err(ParseError::NumberParsing(format!(
                "Failed to parse digit in {}",
                expression
            ))),
        }
    }
}

impl TryFrom<&str> for Expression {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Expression, Self::Error> {
        Self::parse(input)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_expressions() {
        assert_eq!(
            Expression::try_from("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .value(),
            71
        );
        assert_eq!(
            Expression::try_from("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .value(),
            51
        );
        assert_eq!(Expression::try_from("2 * 3 + (4 * 5)").unwrap().value(), 26);
        assert_eq!(
            Expression::try_from("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap()
                .value(),
            437
        );
        assert_eq!(
            Expression::try_from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .value(),
            12240
        );
        assert_eq!(
            Expression::try_from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 ")
                .unwrap()
                .value(),
            13632
        );
    }

    #[test]
    fn test_advanced_expressions() {
        assert_eq!(
            Expression::parse_adv("6 + 9 * 8 + 6").unwrap().value(),
            15 * 14
        );
        assert_eq!(
            Expression::parse_adv("(6 + 9 * 8 + 6) + 1")
                .unwrap()
                .value(),
            15 * 14 + 1
        );
        assert_eq!(
            Expression::parse_adv("1 + (6 + 9 * 8 + 6) + 1")
                .unwrap()
                .value(),
            15 * 14 + 2
        );
        assert_eq!(
            Expression::parse_adv("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .value(),
            231
        );
        assert_eq!(
            Expression::parse_adv("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .value(),
            51
        );
        assert_eq!(
            Expression::parse_adv("2 * 3 + (4 * 5)").unwrap().value(),
            46
        );
        assert_eq!(
            Expression::parse_adv("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap()
                .value(),
            1445
        );
        assert_eq!(
            Expression::parse_adv("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .value(),
            669060
        );
        assert_eq!(
            Expression::parse_adv("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 ")
                .unwrap()
                .value(),
            23340
        );
    }
}
