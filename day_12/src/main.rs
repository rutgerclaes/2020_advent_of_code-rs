extern crate im_rc;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use im_rc::Vector;
use regex::Regex;
use std::convert::TryFrom;

fn main() {
    println!("--- [AoC 2020] Day 12: Rain Risk ---");
    let input = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let actions = input.iter().map(|l| Action::try_from(&l[..]).unwrap());
    let ship = Ship::new();
    let final_position = actions.fold(ship, |mut sh, action| {
        sh.take_action(&action);
        sh
    });

    let result = final_position.x.abs() + final_position.y.abs();

    println!("Result of part one: {}", result);
}

fn part_two(input: &Vector<String>) {
    let actions = input.iter().map(|l| Action::try_from(&l[..]).unwrap());
    let ship = WaypointShip::new(&Point::new(10, 1));
    let final_position = actions.fold(ship, |mut sh, action| {
        sh.take_action(&action);
        sh
    });

    let result = final_position.position.x.abs() + final_position.position.y.abs();

    println!("Result of part two: {}", result);
}

#[derive(Copy, PartialEq, Debug, Clone)]
enum Action {
    Move(i32, Direction),
    TurnLeft(i32),
    TurnRight(i32),
    Forward(i32),
}

impl Action {
    fn applied_to(&self, ship: &Ship) -> (Direction, i32, i32) {
        match self {
            Self::Forward(distance) => (
                ship.direction,
                ship.direction.dx() * distance,
                ship.direction.dy() * distance,
            ),
            Self::Move(distance, direction) => (
                ship.direction,
                distance * direction.dx(),
                distance * direction.dy(),
            ),
            Self::TurnLeft(angle) => (ship.direction.turn_left(angle), 0, 0),
            Self::TurnRight(angle) => (ship.direction.turn_right(angle), 0, 0),
        }
    }

    fn applied_to_waypoint(&self, position: &Point, waypoint: &Point) -> Point {
        match self {
            Self::Forward(_) => *waypoint,
            Self::Move(distance, direction) => waypoint.move_in_direction(direction, distance),
            Self::TurnLeft(angle) => waypoint.rotate_around(position, angle),
            Self::TurnRight(angle) => waypoint.rotate_around(position, &(-1 * angle)),
        }
    }
}

impl TryFrom<&str> for Action {
    type Error = ();

    fn try_from(input: &str) -> Result<Action, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<action>[NSEWLRF])(?P<value>[0-9]+)$").unwrap();
        }

        let action = RE.captures(input).and_then(|matches| {
            let value = matches
                .name("value")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            match matches.name("action").unwrap().as_str() {
                "N" => Some(Action::Move(value, Direction::N)),
                "S" => Some(Action::Move(value, Direction::S)),
                "E" => Some(Action::Move(value, Direction::E)),
                "W" => Some(Action::Move(value, Direction::W)),
                "L" => Some(Action::TurnLeft(value)),
                "R" => Some(Action::TurnRight(value)),
                "F" => Some(Action::Forward(value)),
                _ => None,
            }
        });

        action.ok_or(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn dx(&self) -> i32 {
        match self {
            Self::E => 1,
            Self::W => -1,
            Self::N | Self::S => 0,
        }
    }

    fn dy(&self) -> i32 {
        match self {
            Self::N => 1,
            Self::S => -1,
            Self::W | Self::E => 0,
        }
    }

    fn delta(&self) -> (i32, i32) {
        (self.dx(), self.dy())
    }

    fn turn_left(&self, angle: &i32) -> Direction {
        Direction::from(i32::from(self) + angle)
    }

    fn turn_right(&self, angle: &i32) -> Direction {
        Direction::from(i32::from(self) - angle)
    }
}

impl From<i32> for Direction {
    fn from(angle: i32) -> Direction {
        match angle.rem_euclid(360) {
            45..=135 => Direction::N,
            136..=225 => Direction::W,
            226..=315 => Direction::S,
            _ => Direction::E,
        }
    }
}

impl From<&Direction> for i32 {
    fn from(direction: &Direction) -> i32 {
        match direction {
            Direction::N => 90,
            Direction::E => 0,
            Direction::S => 270,
            Direction::W => 180,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn delta(&self, other: &Point) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }

    fn rotate_around(&self, other: &Point, angle: &i32) -> Point {
        match angle.rem_euclid(360) / 90 {
            0 => *self,
            1 => {
                let (dx, dy) = other.delta(self);
                Point::new(other.x - dy, other.y + dx)
            }
            2 => {
                let (dx, dy) = other.delta(self);
                Point::new(other.x - dx, other.y - dy)
            }
            3 => {
                let (dx, dy) = other.delta(self);
                Point::new(other.x + dy, other.y - dx)
            }
            _ => panic!("Failed to rotate {} degrees", angle),
        }
    }

    fn move_in_direction(&self, direction: &Direction, distance: &i32) -> Point {
        self.move_by_delta(&direction.delta(), distance)
    }

    fn move_by_delta(&self, delta: &(i32, i32), distance: &i32) -> Point {
        Point::new(self.x + distance * delta.0, self.y + distance * delta.1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: Direction::E,
        }
    }

    fn take_action(&mut self, action: &Action) {
        let (new_direction, dx, dy) = action.applied_to(&self);
        self.x += dx;
        self.y += dy;
        self.direction = new_direction;
    }
}

#[derive(Debug)]
struct WaypointShip {
    waypoint: Point,
    position: Point,
}

impl WaypointShip {
    fn new(waypoint: &Point) -> WaypointShip {
        WaypointShip {
            position: Point::new(0, 0),
            waypoint: *waypoint,
        }
    }

    fn take_action(&mut self, action: &Action) {
        match action {
            Action::Forward(distance) => {
                let delta = self.position.delta(&self.waypoint);
                self.position = self.position.move_by_delta(&delta, distance);
                self.waypoint = self.waypoint.move_by_delta(&delta, distance);
            }
            other_action => {
                self.waypoint = other_action.applied_to_waypoint(&self.position, &self.waypoint);
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_direction_from_angles() {
        assert_eq!(Direction::from(0), Direction::E);
        assert_eq!(Direction::from(-44), Direction::E);
        assert_eq!(Direction::from(44), Direction::E);
        assert_eq!(Direction::from(10 + 360 * 2), Direction::E);
        assert_eq!(Direction::from(10 - 360 * 2), Direction::E);
        assert_eq!(Direction::from(-10 - 360 * 2), Direction::E);

        assert_eq!(Direction::from(90), Direction::N);
        assert_eq!(Direction::from(90 + 44), Direction::N);
        assert_eq!(Direction::from(90 - 44), Direction::N);
        assert_eq!(Direction::from(90 - 20 + 360), Direction::N);

        assert_eq!(Direction::from(180), Direction::W);
        assert_eq!(Direction::from(180 - 44), Direction::W);
        assert_eq!(Direction::from(180 + 44), Direction::W);
        assert_eq!(Direction::from(180 + 20 - 360), Direction::W);

        assert_eq!(Direction::from(270), Direction::S);
        assert_eq!(Direction::from(270 + 44), Direction::S);
        assert_eq!(Direction::from(270 - 44), Direction::S);
        assert_eq!(Direction::from(270 - 10 + 360), Direction::S);
    }
}
