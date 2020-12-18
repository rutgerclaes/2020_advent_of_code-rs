extern crate im_rc;

use im_rc::HashSet;
use im_rc::Vector;
use std::cmp;

fn main() {
    println!("--- [AoC 2020] Day 17: Conway Cubes ---");

    let input = utils::read_strings_from_param();

    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> usize {
    let pocket_dimension = PocketDimension::parse(&input);
    let final_dimension = (1..=6).fold(pocket_dimension, |dim, _| dim.evolve());
    final_dimension.nb_of_active_cells()
}

fn part_two(input: &Vector<String>) -> usize {
    let pocket_dimension = PocketDimension4D::parse(&input);
    let final_dimension = (1..=6).fold(pocket_dimension, |dim, _| dim.evolve());
    final_dimension.nb_of_active_cells()
}

struct PocketDimension {
    active_fields: HashSet<Position>,
}

impl PocketDimension {
    fn parse(input: &Vector<String>) -> PocketDimension {
        let active_fields: HashSet<_> = input
            .iter()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, ch)| match ch {
                        '#' => Some(Position(x as i32, y as i32, 0)),
                        _ => None,
                    })
            })
            .collect();

        PocketDimension { active_fields }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = self.ranges();
        for z in min_z..=max_z {
            println!("z={}", z);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    print!(
                        "{}",
                        if self.is_active(&Position(x, y, z)) {
                            "#"
                        } else {
                            "."
                        }
                    )
                }
                println!();
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn ranges(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        self.active_fields.iter().fold(
            ((0, 0), (0, 0), (0, 0)),
            |((min_x, max_x), (min_y, max_y), (min_z, max_z)), position| {
                (
                    (cmp::min(min_x, position.0), cmp::max(max_x, position.0)),
                    (cmp::min(min_y, position.1), cmp::max(max_y, position.1)),
                    (cmp::min(min_z, position.2), cmp::max(max_z, position.2)),
                )
            },
        )
    }

    fn evolve(&self) -> PocketDimension {
        let areas_of_interest = self.active_fields.iter().copied().flat_map(|position| {
            let mut neighbours = position.neighbours();
            neighbours.push_back(position);
            neighbours
        });

        let next_active_positions =
            areas_of_interest.fold(HashSet::new(), |mut active_positions, position| {
                let count = position
                    .neighbours()
                    .iter()
                    .filter(|pos| self.is_active(pos))
                    .count();
                let active = self.is_active(&position);
                let becomes_active =
                    (active && count >= 2 && count <= 3) || (!active && count == 3);
                if becomes_active {
                    active_positions.insert(position);
                }

                active_positions
            });

        PocketDimension {
            active_fields: next_active_positions,
        }
    }

    fn is_active(&self, position: &Position) -> bool {
        self.active_fields.contains(position)
    }

    fn nb_of_active_cells(&self) -> usize {
        self.active_fields.len()
    }
}

struct PocketDimension4D {
    active_fields: HashSet<Position4D>,
}

impl PocketDimension4D {
    fn parse(input: &Vector<String>) -> PocketDimension4D {
        let active_fields: HashSet<_> = input
            .iter()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, ch)| match ch {
                        '#' => Some(Position4D(x as i32, y as i32, 0, 0)),
                        _ => None,
                    })
            })
            .collect();

        PocketDimension4D { active_fields }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z), (min_w, max_w)) = self.ranges();
        for w in min_w..=max_w {
            for z in min_z..=max_z {
                println!("z={}, w={}", z, w);
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        print!(
                            "{}",
                            if self.is_active(&Position4D(x, y, z, w)) {
                                "#"
                            } else {
                                "."
                            }
                        )
                    }
                    println!();
                }
                println!();
            }
        }
    }

    #[allow(dead_code, clippy::type_complexity)]
    fn ranges(&self) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
        self.active_fields.iter().fold(
            ((0, 0), (0, 0), (0, 0), (0, 0)),
            |((min_x, max_x), (min_y, max_y), (min_z, max_z), (min_w, max_w)), position| {
                (
                    (cmp::min(min_x, position.0), cmp::max(max_x, position.0)),
                    (cmp::min(min_y, position.1), cmp::max(max_y, position.1)),
                    (cmp::min(min_z, position.2), cmp::max(max_z, position.2)),
                    (cmp::min(min_w, position.3), cmp::max(max_w, position.3)),
                )
            },
        )
    }

    fn evolve(&self) -> PocketDimension4D {
        let areas_of_interest = self.active_fields.iter().copied().flat_map(|position| {
            let mut neighbours = position.neighbours();
            neighbours.push_back(position);
            neighbours
        });

        let next_active_positions =
            areas_of_interest.fold(HashSet::new(), |mut active_positions, position| {
                let count = position
                    .neighbours()
                    .iter()
                    .filter(|pos| self.is_active(pos))
                    .count();
                let active = self.is_active(&position);
                let becomes_active =
                    (active && count >= 2 && count <= 3) || (!active && count == 3);
                if becomes_active {
                    active_positions.insert(position);
                }

                active_positions
            });

        PocketDimension4D {
            active_fields: next_active_positions,
        }
    }

    fn is_active(&self, position: &Position4D) -> bool {
        self.active_fields.contains(position)
    }

    fn nb_of_active_cells(&self) -> usize {
        self.active_fields.len()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position(i32, i32, i32);

impl Position {
    fn neighbour_matrix() -> Vector<(i32, i32, i32)> {
        (-1..=1)
            .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
            .filter(|(x, y, z)| x != &0 || y != &0 || z != &0)
            .collect()
    }

    fn neighbours(&self) -> Vector<Position> {
        Self::neighbour_matrix()
            .iter()
            .map(|(x, y, z)| self.shift(x, y, z))
            .collect()
    }

    fn shift(&self, dx: &i32, dy: &i32, dz: &i32) -> Position {
        Position(self.0 + dx, self.1 + dy, self.2 + dz)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position4D(i32, i32, i32, i32);

impl Position4D {
    fn neighbour_matrix() -> Vector<(i32, i32, i32, i32)> {
        (-1..=1)
            .flat_map(|x| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (x, y, z, w)))
                })
            })
            .filter(|(x, y, z, w)| x != &0 || y != &0 || z != &0 || w != &0)
            .collect()
    }

    fn neighbours(&self) -> Vector<Position4D> {
        Self::neighbour_matrix()
            .iter()
            .map(|(x, y, z, w)| self.shift(x, y, z, w))
            .collect()
    }

    fn shift(&self, dx: &i32, dy: &i32, dz: &i32, dw: &i32) -> Position4D {
        Position4D(self.0 + dx, self.1 + dy, self.2 + dz, self.3 + dw)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_position_neighbours_offsets() {
        let position = Position(0, 0, 0);
        assert_eq!(position.neighbours().len(), 26);
    }

    #[test]
    fn test_position4d_neighbours_offsets() {
        let position = Position4D(0, 0, 0, 0);
        assert_eq!(position.neighbours().len(), 80);
    }
}
