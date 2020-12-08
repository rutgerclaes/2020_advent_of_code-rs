extern crate im_rc;

use im_rc::vector;
use im_rc::Vector;
use std::convert::TryFrom;

fn main() {
    println!("--- [AoC 2020] Day 3: Toboggan Trajectory ---");
    let input: String = utils::read_string_from_params();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let map = TobogganMap::try_from(input).unwrap();
    let path = (0..map.height()).map(|y| (y * 3, y));

    let nb_of_trees = path.filter(|(x, y)| map.has_tree_at(*x, *y)).count();

    println!("Solution for part one: {}", nb_of_trees);
}

fn part_two(input: &str) {
    let map = TobogganMap::try_from(input).unwrap();
    let paths = vector!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2));
    let prod: usize = paths
        .iter()
        .map(|(dx, dy)| map.count_trees_on_path(map.traverse(*dx, *dy)))
        .product();

    println!("Solution for part two: {}", prod);
}

#[derive(PartialEq, Clone)]
enum TobogganMapElement {
    Empty,
    Tree,
}

impl TobogganMapElement {
    fn is_tree(&self) -> bool {
        self == &Self::Tree
    }
}

struct TobogganMap {
    cells: Vector<Vector<TobogganMapElement>>,
}

impl TryFrom<char> for TobogganMapElement {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Tree),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for TobogganMap {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let cells: Vector<Vector<TobogganMapElement>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| TobogganMapElement::try_from(c).unwrap())
                    .collect()
            })
            .collect();
        Ok(TobogganMap { cells })
    }
}

impl TobogganMap {
    fn element_at(&self, x: usize, y: usize) -> Option<&TobogganMapElement> {
        let normalized_x: Option<usize> = self.cells.get(y).map(|row| x % row.len());
        normalized_x.map(|xv| self.cells.get(y).unwrap().get(xv).unwrap())
    }

    fn has_tree_at(&self, x: usize, y: usize) -> bool {
        self.element_at(x, y).map(|e| e.is_tree()).unwrap_or(false)
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn traverse(&self, delta_x: usize, delta_y: usize) -> Vector<(usize, usize)> {
        (0..self.height())
            .step_by(delta_y)
            .map(|y| (delta_x * y / delta_y, y))
            .collect()
    }

    fn count_trees_on_path(&self, path: Vector<(usize, usize)>) -> usize {
        path.iter()
            .filter(|(x, y)| self.has_tree_at(*x, *y))
            .count()
    }
}
