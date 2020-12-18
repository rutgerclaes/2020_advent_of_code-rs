extern crate im_rc;

use im_rc::HashMap;
use im_rc::Vector;
use itertools::Itertools;
use std::convert::TryFrom;
use std::fmt;

fn main() {
    println!("--- [AoC 2020] Day 16: Ticket Translation ---");

    let input = utils::read_strings_from_param();
    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two_bis(&input).unwrap());
}

fn part_one(input: &Vector<String>) -> u32 {
    let mut input_iterator = input.iter();
    let definitions: Vector<_> = input_iterator
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| FieldDefinition::try_from(&line[..]).unwrap())
        .collect();

    let mut tickets: Vector<_> = input_iterator
        .filter(|line| {
            line.chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        })
        .map(|line| Ticket::try_from(&line[..]).unwrap())
        .collect();

    let _own_ticket = tickets.pop_front().unwrap();

    tickets
        .iter()
        .flat_map(|t| t.field_values.iter())
        .filter(|field| definitions.iter().all(|def| !def.validate(field)))
        .sum()
}

fn part_two_bis(input: &Vector<String>) -> Option<u64> {
    let mut input_iterator = input.iter();
    let definitions: Vector<_> = input_iterator
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| FieldDefinition::try_from(&line[..]).unwrap())
        .collect();

    let mut tickets: Vector<_> = input_iterator
        .filter(|line| {
            line.chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        })
        .map(|line| Ticket::try_from(&line[..]).unwrap())
        .collect();

    let own_ticket = tickets.pop_front().unwrap();

    let valid_tickets: Vector<_> = tickets
        .iter()
        .filter(|ticket| {
            ticket
                .field_values
                .iter()
                .all(|value| definitions.iter().any(|def| def.validate(value)))
        })
        .collect();

    let departure_fields: Vector<_> = definitions
        .iter()
        .filter(|def| def.name.starts_with("departure"))
        .collect();

    let positions: Vector<_> = (0..definitions.len()).collect();

    let possible_assignments: Vector<_> = positions
        .iter()
        .flat_map(|position| {
            let values: Vector<_> = valid_tickets
                .iter()
                .map(|t| *t.field_values.get(*position).unwrap())
                .collect();
            let possibilities: Vector<_> = definitions
                .iter()
                .filter_map(|def| {
                    Some((&def.name[..], *position)).filter(|_| def.validate_all(&values))
                })
                .collect();
            possibilities
        })
        .collect();

    let initial_assignments: HashMap<&str, Option<usize>> = definitions
        .iter()
        .map(|field| (&field.name[..], None))
        .collect();

    assign_next(initial_assignments, possible_assignments).map(|result| {
        departure_fields
            .iter()
            .map(|field| {
                let position = result.get(&field.name[..]).unwrap();
                let field_value: u64 = *own_ticket.field_values.get(*position).unwrap() as u64;
                field_value
            })
            .product()
    })
}

fn assign_next<'a>(
    current_assignments: HashMap<&'a str, Option<usize>>,
    possible_assignments: Vector<(&'a str, usize)>,
) -> Option<HashMap<&'a str, usize>> {
    let possibilities: HashMap<&str, usize> = possible_assignments
        .iter()
        .fold(HashMap::new(), |counts, (field, _)| {
            counts.update_with(field, 1, |old, new| old + new)
        });

    let next_possible_field = current_assignments
        .iter()
        .filter_map(|(&field, position)| Some(field).filter(|_| position.is_none()))
        .min_by_key(|&field| possibilities.get(field));

    match next_possible_field {
        None => Some(
            current_assignments
                .iter()
                .map(|(&field, position)| (field, position.unwrap()))
                .collect(),
        ),
        Some(next_field) => {
            let possible_positions_for_field: Vector<_> = possible_assignments
                .iter()
                .filter(|(field, _)| field == &next_field)
                .copied()
                .collect();

            possible_positions_for_field
                .iter()
                .find_map(|(_, next_position)| {
                    let updated_assignments =
                        current_assignments.update(next_field, Some(*next_position));
                    let updated_pos_assignments: Vector<_> = possible_assignments
                        .iter()
                        .filter(|(possible_field, possible_position)| {
                            possible_field != &next_field && possible_position != next_position
                        })
                        .copied()
                        .collect();

                    assign_next(updated_assignments, updated_pos_assignments)
                })
        }
    }
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
struct FieldDefinition {
    name: String,
    ranges: Vector<(u32, u32)>,
}

impl FieldDefinition {
    fn new(name: &str, ranges: Vector<(u32, u32)>) -> FieldDefinition {
        FieldDefinition {
            name: name.to_owned(),
            ranges,
        }
    }

    fn validate(&self, value: &u32) -> bool {
        self.ranges
            .iter()
            .any(|(min, max)| min <= value && max >= value)
    }

    fn validate_all(&self, values: &Vector<u32>) -> bool {
        values.iter().all(|value| self.validate(value))
    }
}

impl fmt::Display for FieldDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ranges = match self.ranges.head() {
            Some((min, max)) => self
                .ranges
                .iter()
                .skip(1)
                .fold(format!("{}-{}", min, max), |buffer, (min, max)| {
                    format!("{} or {}-{}", buffer, min, max)
                }),
            None => String::new(),
        };
        write!(f, "{}: {}", self.name, ranges)
    }
}

impl TryFrom<&str> for FieldDefinition {
    type Error = ();

    fn try_from(input: &str) -> Result<FieldDefinition, Self::Error> {
        let (name, ranges_input) = input.split(": ").collect_tuple().ok_or(())?;
        let ranges: Vector<(u32, u32)> = ranges_input
            .split(" or ")
            .map(|range_text| {
                range_text
                    .split('-')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        Ok(FieldDefinition::new(name, ranges))
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    field_values: Vector<u32>,
}

impl Ticket {
    fn new(fields: &Vector<u32>) -> Ticket {
        Ticket {
            field_values: fields.iter().copied().collect(),
        }
    }
}

impl TryFrom<&str> for Ticket {
    type Error = ();

    fn try_from(input: &str) -> Result<Ticket, Self::Error> {
        let fields = input
            .split(',')
            .map(|p| p.parse::<u32>().unwrap())
            .collect();
        Ok(Ticket::new(&fields))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use im_rc::vector;

    #[test]
    fn test_field_definition_parsing() {
        let input = "class: 1-3 or 5-7";
        let field_def = FieldDefinition::try_from(input);
        assert_eq!(
            field_def,
            Ok(FieldDefinition::new("class", vector!((1, 3), (5, 7))))
        );
    }
}
