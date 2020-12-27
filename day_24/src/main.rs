extern crate im_rc;
extern crate log;
extern crate pretty_env_logger;

use im_rc::HashSet;
use im_rc::Vector;
use itertools::Itertools;
use log::{debug, info, trace};
use pbr::ProgressBar;

fn main() {
    pretty_env_logger::init();
    info!("--- [AoC 2020] Day 24: Lobby Layout ---");

    let input = utils::read_strings_from_param();
    debug!("Number of directions: {}", input.len());

    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> usize {
    let mut progress = ProgressBar::new(input.len() as u64);
    let start = (0, 0);
    let floor = input.iter().fold(Floor::new(), |mut floor, directions| {
        progress.inc();
        floor.toggle(Floor::navigate(&start, &HexDirection::parse(&directions)));
        floor
    });
    progress.finish();
    floor.nb_of_black_tiles()
}

fn part_two(input: &Vector<String>) -> usize {
    let start = (0, 0);
    let floor = input.iter().fold(Floor::new(), |mut floor, directions| {
        floor.toggle(Floor::navigate(&start, &HexDirection::parse(&directions)));
        floor
    });

    let mut progress = ProgressBar::new(100);
    let final_floor = (0..100).fold(floor, |mut floor, iteration| {
        floor.flip_all();
        progress.inc();
        trace!("Day {}: {}", iteration + 1, floor.nb_of_black_tiles());
        floor
    });

    progress.finish();
    final_floor.nb_of_black_tiles()
}

#[derive(Copy, Debug, PartialEq, Clone)]
enum HexDirection {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexDirection {
    fn parse(input: &str) -> Vector<HexDirection> {
        input
            .chars()
            .batching(|it| match it.next() {
                Some('n') => match it.next() {
                    Some('e') => Some(Self::NorthEast),
                    Some('w') => Some(Self::NorthWest),
                    Some(c) => panic!("Unsupported direction after 's': {}", c),
                    None => panic!("No second char after 's'"),
                },
                Some('s') => match it.next() {
                    Some('e') => Some(Self::SouthEast),
                    Some('w') => Some(Self::SouthWest),
                    Some(c) => panic!("Unsupported direction after 's': {}", c),
                    None => panic!("No second char after 's'"),
                },
                Some('e') => Some(Self::East),
                Some('w') => Some(Self::West),
                Some(c) => panic!("Unsupported char without prefix: {}", c),
                None => None,
            })
            .collect()
    }

    fn coordinate_changes(&self) -> (Coordinate, Coordinate) {
        match self {
            Self::East => (1, 0),
            Self::SouthEast => (1, -1),
            Self::SouthWest => (0, -1),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, 1),
            Self::NorthEast => (0, 1),
        }
    }
}

struct Floor {
    black_tiles: HashSet<Point>,
    max_x: Coordinate,
    max_y: Coordinate,
    min_x: Coordinate,
    min_y: Coordinate,
}

type Coordinate = i16;
type Point = (Coordinate, Coordinate);

const DIRECTIONS: [HexDirection; 6] = [
    HexDirection::East,
    HexDirection::NorthEast,
    HexDirection::NorthWest,
    HexDirection::SouthEast,
    HexDirection::SouthWest,
    HexDirection::West,
];

impl Floor {
    fn new() -> Floor {
        Floor {
            black_tiles: HashSet::new(),
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
        }
    }

    fn flip_all(&mut self) {
        let next_blacks =
            ((self.min_y - 1)..=(self.max_y + 1)).fold(HashSet::new(), |blacks, y| {
                ((self.min_x - 1)..=(self.max_x + 1)).fold(blacks, |mut blacks, x| {
                    let black = self.black_tiles.contains(&(x, y));
                    let black_neighbours = self.nb_of_black_neighbours(&(x, y));

                    if black {
                        let becomes_white = black_neighbours == 0 || black_neighbours > 2;
                        if !becomes_white {
                            blacks.insert((x, y));
                            self.update_bounds((x, y));
                        }
                    } else {
                        let becomes_black = black_neighbours == 2;
                        if becomes_black {
                            blacks.insert((x, y));
                            self.update_bounds((x, y));
                        }
                    }

                    blacks
                })
            });

        self.black_tiles = next_blacks;
    }

    fn nb_of_black_neighbours(&self, pos: &Point) -> usize {
        DIRECTIONS
            .iter()
            .map(|dir| dir.coordinate_changes())
            .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
            .filter(|point| self.black_tiles.contains(point))
            .count()
    }

    fn navigate(start: &Point, directions: &Vector<HexDirection>) -> Point {
        directions
            .iter()
            .map(|dir| dir.coordinate_changes())
            .fold(*start, |(x, y), (dx, dy)| (x + dx, y + dy))
    }

    fn toggle(&mut self, point: Point) {
        if self.black_tiles.contains(&point) {
            self.black_tiles.remove(&point);
        } else {
            self.update_bounds(point);
            self.black_tiles.insert(point);
        }
    }

    fn nb_of_black_tiles(&self) -> usize {
        self.black_tiles.len()
    }

    fn update_bounds(&mut self, point: Point) {
        self.max_x = std::cmp::max(self.max_x, point.0);
        self.max_y = std::cmp::max(self.max_y, point.1);
        self.min_x = std::cmp::min(self.min_x, point.0);
        self.min_y = std::cmp::min(self.min_y, point.1);
    }
}
