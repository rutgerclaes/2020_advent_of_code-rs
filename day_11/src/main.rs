extern crate im_rc;

use im_rc::Vector;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

fn main() {
    println!("--- [AoC 2020] Day 11: Seating System ---");
    let input = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let layout = Layout::try_from(input).unwrap();

    let states = itertools::iterate(Some(layout), |l| l.as_ref().and_then(|l| l.next()))
        .take_while(|l| l.is_some())
        .filter_map(|l| l);

    let final_state = states.last().unwrap();
    let result = final_state.nb_of_occupied_seats();

    println!("Solution to part one: {}", result);
}

fn part_two(input: &Vector<String>) {
    let layout = Layout::try_from(input).unwrap();

    let states = itertools::iterate(Some(layout), |l| l.as_ref().and_then(|l| l.next_v2()))
        .take_while(|l| l.is_some())
        .filter_map(|l| l);

    let final_state = states.last().unwrap();
    let result = final_state.nb_of_occupied_seats();
    println!("Solution to part two: {}", result);
}

#[derive(Clone, PartialEq, Copy, Debug, Eq, PartialOrd, Ord)]
enum Position {
    Seat(bool),
    Floor,
}

impl Position {
    fn is_occupied(&self) -> bool {
        match self {
            Self::Seat(true) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Seat(true) => write!(f, "#"),
            Self::Seat(false) => write!(f, "L"),
            Self::Floor => write!(f, "."),
        }
    }
}

impl TryFrom<&char> for Position {
    type Error = ();

    fn try_from(input: &char) -> Result<Position, Self::Error> {
        match input {
            'L' => Ok(Self::Seat(false)),
            '#' => Ok(Self::Seat(true)),
            '.' => Ok(Self::Floor),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Layout {
    seats: Vector<Vector<Position>>,
}

impl Layout {
    fn height(&self) -> usize {
        self.seats.len()
    }

    fn width(&self) -> usize {
        self.seats.head().map(|row| row.len()).unwrap_or(0)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Position> {
        self.seats.get(y).and_then(|r| r.get(x))
    }

    fn nb_of_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .map(|r| r.iter())
            .flatten()
            .filter(|seat| match seat {
                Position::Seat(true) => true,
                _ => false,
            })
            .count()
    }

    fn get_surroundings(&self, x: usize, y: usize) -> Vector<&Position> {
        let minus_one: i32 = -1;
        let plus_one: i32 = 1;
        let positions = (minus_one..=plus_one)
            .flat_map(|dy| (minus_one..=plus_one).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| !(dx == &0 && dy == &0))
            .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy));

        positions
            .filter_map(|(x, y)| {
                if let Ok(usize_x) = usize::try_from(x) {
                    if let Ok(usize_y) = usize::try_from(y) {
                        Some((usize_x, usize_y))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter_map(|(x, y)| self.seats.get(y).and_then(|r| r.get(x)))
            .collect()
    }

    fn get_directions(&self, x: usize, y: usize) -> Vector<&Position> {
        let minus_one: i8 = -1;
        let plus_one: i8 = 1;
        let directions = (minus_one..=plus_one)
            .flat_map(|dy| (minus_one..=plus_one).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| !(dx == &0 && dy == &0));

        directions
            .filter_map(|(dx, dy)| self.get_direction(x, y, dx, dy))
            .collect()
    }

    fn get_direction(&self, x: usize, y: usize, dx: i8, dy: i8) -> Option<&Position> {
        let qx: i32 = x as i32 + dx as i32;
        let qy: i32 = y as i32 + dy as i32;

        if qx >= 0 && qy >= 0 && qx < self.width() as i32 && qy < self.height() as i32 {
            let safe_x: usize = qx.try_into().unwrap();
            let safe_y: usize = qy.try_into().unwrap();
            self.get(safe_x, safe_y)
                .filter(|pos| match pos {
                    Position::Floor => false,
                    Position::Seat(_) => true,
                })
                .or_else(|| self.get_direction(safe_x, safe_y, dx, dy))
        } else {
            None
        }
    }

    fn next(&self) -> Option<Layout> {
        let changes: Vector<_> = self
            .seats
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, seat)| match seat {
                        Position::Seat(true) => {
                            let surroundings = self.get_surroundings(x, y);
                            let needs_change = surroundings
                                .iter()
                                .filter(|position| position.is_occupied())
                                .count()
                                >= 4;

                            if needs_change {
                                Some((x, y, Position::Seat(false)))
                            } else {
                                None
                            }
                        }
                        Position::Seat(false) => {
                            let surroundings = self.get_surroundings(x, y);
                            let needs_change =
                                surroundings.iter().all(|position| !position.is_occupied());
                            if needs_change {
                                Some((x, y, Position::Seat(true)))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
            })
            .collect();

        if changes.is_empty() {
            None
        } else {
            Some(self.apply_changes(&changes))
        }
    }

    fn next_v2(&self) -> Option<Layout> {
        let changes: Vector<_> = self
            .seats
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, seat)| match seat {
                        Position::Seat(true) => {
                            let surroundings = self.get_directions(x, y);
                            let needs_change = surroundings
                                .iter()
                                .filter(|position| position.is_occupied())
                                .count()
                                >= 5;

                            if needs_change {
                                Some((x, y, Position::Seat(false)))
                            } else {
                                None
                            }
                        }
                        Position::Seat(false) => {
                            let surroundings = self.get_directions(x, y);
                            let needs_change =
                                surroundings.iter().all(|position| !position.is_occupied());
                            if needs_change {
                                Some((x, y, Position::Seat(true)))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
            })
            .collect();

        if changes.is_empty() {
            None
        } else {
            Some(self.apply_changes(&changes))
        }
    }

    fn apply_changes(&self, changes: &Vector<(usize, usize, Position)>) -> Layout {
        let new_seats: Vector<Vector<Position>> = self
            .seats
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let new_row: Vector<Position> = row
                    .iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        changes
                            .iter()
                            .find_map(|(change_x, change_y, new)| {
                                if *change_x == x && *change_y == y {
                                    Some(new)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(seat)
                    })
                    .copied()
                    .collect();
                new_row
            })
            .collect();

        Layout { seats: new_seats }
    }
}

impl TryFrom<&Vector<String>> for Layout {
    type Error = ();

    fn try_from(input: &Vector<String>) -> Result<Layout, Self::Error> {
        let positions = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| Position::try_from(&c).unwrap())
                    .collect()
            })
            .collect();
        Ok(Layout { seats: positions })
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                output = format!("{}{}", output, self.get(x, y).unwrap());
            }
            output = format!("{}\n", output);
        }
        writeln!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use im_rc::vector;

    #[test]
    fn test_layout_surroundings() {
        let seats = vector![
            vector![
                Position::Seat(false),
                Position::Floor,
                Position::Seat(false),
            ],
            vector![
                Position::Seat(false),
                Position::Seat(true),
                Position::Seat(false),
            ],
            vector![Position::Floor, Position::Seat(false), Position::Floor],
        ];

        let layout = Layout { seats };

        let mut surroundings = layout.get_surroundings(1, 1);
        surroundings.sort();
        let mut expected = vector!(
            &Position::Seat(false),
            &Position::Floor,
            &Position::Seat(false),
            &Position::Seat(false),
            &Position::Seat(false),
            &Position::Floor,
            &Position::Seat(false),
            &Position::Floor
        );
        expected.sort();

        // assert_eq!(surroundings.len(), 8);
        assert_eq!(surroundings, expected);
    }

    fn parse(input: &str) -> Layout {
        let lines: Vector<String> = input.lines().map(|s| s.to_owned()).collect();
        Layout::try_from(&lines).unwrap()
    }

    #[test]
    fn test_layout_directions_1() {
        let input = ".............\n\
                     .L.L.#.#.#.#.\n\
                     .............";

        let layout = parse(input);
        println!("{}", layout);
        let mut surroundings = layout.get_directions(1, 1);
        surroundings.sort();
        let mut expected = vector!(&Position::Seat(false));
        expected.sort();
        assert_eq!(surroundings, expected);
    }

    #[test]
    fn test_layout_directions_2() {
        let input = "#.##\n\
                     #.##\n\
                     .#..\n\
                     #.#L";

        let layout = parse(input);
        let mut surroundings = layout.get_directions(0, 0);
        surroundings.sort();
        let mut expected = vector!(
            &Position::Seat(true),
            &Position::Seat(false),
            &Position::Seat(true)
        );
        expected.sort();
        assert_eq!(surroundings, expected);
    }

    #[test]
    fn test_evolution_v2() {
        let input = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";
        let layout = parse(input);

        let step1 = "#.##.##.##\n\
                     #######.##\n\
                     #.#.#..#..\n\
                     ####.##.##\n\
                     #.##.##.##\n\
                     #.#####.##\n\
                     ..#.#.....\n\
                     ##########\n\
                     #.######.#\n\
                     #.#####.##";
        let result1 = layout.next_v2().unwrap();
        assert_eq!(result1, parse(step1));

        let step2 = "#.LL.LL.L#\n\
                     #LLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLL#\n\
                     #.LLLLLL.L\n\
                     #.LLLLL.L#";
        let result2 = result1.next_v2().unwrap();
        assert_eq!(result2, parse(step2));
    }
}
