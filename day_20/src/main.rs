extern crate im_rc;

use im_rc::get_in;
use im_rc::hashmap;
use im_rc::vector;
use im_rc::HashMap;
use im_rc::HashSet;
use im_rc::Vector;
use std::cmp;
use std::fmt;
use std::iter::FromIterator;

fn main() {
    println!("--- [AoC 2020] Day 20: Jurrasic Jigsaw ---");

    let input = utils::read_strings_from_param();

    // println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> i64 {
    let (mut tiles, last_buffer) = input.iter().fold(
        (HashMap::new(), Vector::new()),
        |(mut tiles, mut buffer), line| {
            if line.is_empty() {
                let tile = Tile::parse(&buffer);
                tiles.insert(tile.id, tile);
                (tiles, Vector::new())
            } else {
                buffer.push_back(line);
                (tiles, buffer)
            }
        },
    );

    if !last_buffer.is_empty() {
        let tile = Tile::parse(&last_buffer);
        tiles.insert(tile.id, tile);
    }

    let tile = tiles.values().nth(0).unwrap().clone();

    let mut tiles_to_fit: HashSet<&i32> = tiles.keys().filter(|k| *k != &tile.id).collect();
    let mut grid = TileGrid::new(tile);

    while !tiles_to_fit.is_empty() {
        if let Some(next) = tiles_to_fit
            .iter()
            .find(|tile_id| grid.insert(tiles.get(tile_id).unwrap()))
        {
            println!("Fitted {} ({} left)", next, tiles_to_fit.len() - 1);
            println!("{}", grid);
            tiles_to_fit.remove(*next);
        } else {
            panic!("Could not find next tile to insert");
        }
    }

    grid.corner_ids().iter().map(|&i| i as i64).product()
}

fn part_two(input: &Vector<String>) -> usize {
    let (mut tiles, last_buffer) = input.iter().fold(
        (HashMap::new(), Vector::new()),
        |(mut tiles, mut buffer), line| {
            if line.is_empty() {
                let tile = Tile::parse(&buffer);
                tiles.insert(tile.id, tile);
                (tiles, Vector::new())
            } else {
                buffer.push_back(line);
                (tiles, buffer)
            }
        },
    );

    if !last_buffer.is_empty() {
        let tile = Tile::parse(&last_buffer);
        tiles.insert(tile.id, tile);
    }

    let tile = tiles.values().next().unwrap().clone();
    let tile = tiles.get(&3413).unwrap().clone();
    // let tile = tiles.get(&1951).unwrap().clone();
    let mut grid = TileGrid::new(tile);
    grid.insert_all(tiles);

    println!("Completed grid!");
    println!("{}", grid);

    let pattern_strings = vector!(
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "
    );

    let pattern =
        pattern_strings
            .iter()
            .enumerate()
            .fold(Vector::new(), |mut positions, (y, line)| {
                line.chars()
                    .enumerate()
                    .fold(positions, |mut positions, (x, ch)| {
                        if ch == '#' {
                            positions.push_back((x, y));
                        }
                        positions
                    })
            });

    let image = Image::new(&grid);
    let (monster_coordinates, image_variant) = image
        .flip()
        .variations()
        .find_map(|variation| {
            let positions = variation.search(&pattern);
            if positions.is_empty() {
                None
            } else {
                Some((positions, variation))
            }
        })
        .unwrap();

    image_variant.count_pos_without_monster(&pattern, &monster_coordinates)
}

#[derive(Debug)]
struct TileGrid {
    tiles: HashMap<(i32, i32), Tile>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl fmt::Display for TileGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                write!(
                    f,
                    " {} ",
                    self.tiles
                        .get(&(x, y))
                        .map(|t| format!(" {:4}  ", t.id))
                        .unwrap_or(format!("({:+},{:+})", x, y))
                );
            }
            writeln!(f);
        }
        write!(f, "")
    }
}

impl TileGrid {
    fn normal_get(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get(&(x + self.min_x, y + self.min_y))
    }

    fn state_at(&self, x: i32, y: i32) -> bool {
        let grid_y = y / 8;
        let grid_x = x / 8;
        let tile_y = y - grid_y * 8 + 1;
        let tile_x = x - grid_x * 8 + 1;

        self.normal_get(grid_x, grid_y)
            .unwrap()
            .positions
            .contains(&TilePosition(tile_x, tile_y))
    }

    fn state_width(&self) -> i32 {
        self.width() * 8
    }

    fn state_height(&self) -> i32 {
        self.height() * 8
    }

    fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }

    fn insert_all(&mut self, tiles: HashMap<i32, Tile>) {
        let current_tiles: HashSet<i32> = self.tiles.values().map(|tile| tile.id).collect();
        let mut tiles_to_fit: HashSet<i32> = tiles
            .keys()
            .copied()
            .collect::<HashSet<_>>()
            .difference(current_tiles);

        while !tiles_to_fit.is_empty() {
            if let Some(&next) = tiles_to_fit
                .iter()
                .find(|tile_id| self.insert(tiles.get(tile_id).unwrap()))
            {
                tiles_to_fit.remove(&next);
            } else {
                panic!("Could not find next tile to insert");
            }
        }
    }

    fn insert_at(&mut self, position: (i32, i32), tile: Tile) {
        // println!("Inserting {} at {:?}", tile.id, position);
        self.tiles.insert(position, tile);
        self.min_x = cmp::min(self.min_x, position.0);
        self.max_x = cmp::max(self.max_x, position.0);
        self.min_y = cmp::min(self.min_y, position.1);
        self.max_y = cmp::max(self.max_y, position.1);
    }

    fn insert(&mut self, tile: &Tile) -> bool {
        if let Some((position, variation)) = tile.variations().find_map(|variation| {
            self.positions_to_fill()
                .iter()
                .find(|(x, y)| self.fits((*x, *y), &variation))
                .map(|(x, y)| ((*x, *y), variation))
        }) {
            self.insert_at(position, variation);
            true
        } else {
            false
        }
    }

    fn positions_to_fill(&self) -> HashSet<(i32, i32)> {
        let positions: Vector<_> = self
            .tiles
            .keys()
            .flat_map(|(x, y)| vector!((x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)))
            .filter(|(x, y)| !self.tiles.contains_key(&(*x, *y)))
            .collect();

        let (doubles, uniques) = positions.iter().copied().fold(
            (HashSet::new(), HashSet::new()),
            |(mut doubles, mut uniques), position| {
                if uniques.contains(&position) {
                    doubles.insert(position);
                } else {
                    uniques.insert(position);
                }
                (doubles, uniques)
            },
        );

        if doubles.is_empty() {
            uniques
        } else {
            doubles
        }
    }

    fn fits(&self, position: (i32, i32), tile: &Tile) -> bool {
        if let Some(above) = self.tiles.get(&(position.0, position.1 - 1)) {
            let bottom = above.bottom();
            let top = tile.top();

            if bottom != top {
                return false;
            }
        }

        if let Some(below) = self.tiles.get(&(position.0, position.1 + 1)) {
            let top = below.top();
            let bottom = tile.bottom();

            if top != bottom {
                return false;
            }
        }

        if let Some(to_left) = self.tiles.get(&(position.0 - 1, position.1)) {
            let right = to_left.right();
            let left = tile.left();

            if left != right {
                return false;
            }
        }

        if let Some(to_right) = self.tiles.get(&(position.0 + 1, position.1)) {
            let left = to_right.left();
            let right = tile.right();

            if left != right {
                return false;
            }
        }
        true
    }

    fn corners(&self) -> Vector<(i32, i32)> {
        vector!(
            (self.min_x, self.min_y),
            (self.min_x, self.max_y),
            (self.max_x, self.min_y),
            (self.max_x, self.max_y)
        )
    }

    fn corner_ids(&self) -> Vector<i32> {
        self.corners()
            .iter()
            .map(|pos| self.tiles.get(pos).map(|tile| tile.id).unwrap_or(0))
            .collect()
    }

    fn new(initial_tile: Tile) -> TileGrid {
        TileGrid {
            tiles: hashmap!( (0,0) => initial_tile ),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TilePosition(i32, i32);

impl TilePosition {
    fn rotate(&self, max_y: i32) -> TilePosition {
        TilePosition(self.1, max_y - self.0)
    }

    fn flip(&self, max_x: i32) -> TilePosition {
        TilePosition(max_x - self.0, self.1)
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: i32,
    positions: HashSet<TilePosition>,
    width: i32,
    height: i32,
}

struct TileIterator {
    tile: Tile,
    rotated: i8,
    flipped: bool,
}

impl Iterator for TileIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        if !self.flipped {
            if self.rotated <= 3 {
                let result = Some(self.tile.clone());
                self.tile = self.tile.rotate();
                self.rotated += 1;
                result
            } else {
                let result = Some(self.tile.clone());
                self.tile = self.tile.flip();
                self.rotated = 0;
                self.flipped = true;
                result
            }
        } else if self.rotated < 3 {
            let result = Some(self.tile.clone());
            self.tile = self.tile.rotate();
            self.rotated += 1;
            result
        } else {
            None
        }
    }
}

impl Tile {
    fn rotate(&self) -> Tile {
        Tile {
            id: self.id,
            positions: self
                .positions
                .iter()
                .map(|pos| pos.rotate(self.height - 1))
                .collect(),
            width: self.height,
            height: self.width,
        }
    }

    fn flip(&self) -> Tile {
        Tile {
            id: self.id,
            positions: self
                .positions
                .iter()
                .map(|pos| pos.flip(self.width - 1))
                .collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn variations(&self) -> impl Iterator<Item = Tile> {
        TileIterator {
            tile: self.clone(),
            rotated: 0,
            flipped: false,
        }
    }

    fn bottom(&self) -> String {
        let chars = (0..self.width).map(|i| {
            if self.positions.contains(&TilePosition(i, self.height - 1)) {
                '#'
            } else {
                '.'
            }
        });
        String::from_iter(chars)
    }

    fn top(&self) -> String {
        let chars = (0..self.width).map(|i| {
            if self.positions.contains(&TilePosition(i, 0)) {
                '#'
            } else {
                '.'
            }
        });
        String::from_iter(chars)
    }

    fn right(&self) -> String {
        let chars = (0..self.height).map(|i| {
            if self.positions.contains(&TilePosition(self.width - 1, i)) {
                '#'
            } else {
                '.'
            }
        });
        String::from_iter(chars)
    }

    fn left(&self) -> String {
        let chars = (0..self.height).map(|i| {
            if self.positions.contains(&TilePosition(0, i)) {
                '#'
            } else {
                '.'
            }
        });
        String::from_iter(chars)
    }

    fn parse(input: &Vector<&String>) -> Tile {
        let mut head = String::from(*input.head().unwrap());
        head.retain(|ch| ch.is_ascii_digit());
        let id = head.parse::<i32>().unwrap();

        let (positions, width, height) = input
            .iter()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .enumerate()
            .fold(
                (HashSet::new(), 0, 0),
                |(points, width, height), (y, line)| {
                    let (updated_points, line_width) = line.chars().enumerate().fold(
                        (points, width),
                        |(points, width), (x, ch)| {
                            let updated_points = if ch == '#' {
                                points.update(TilePosition(x as i32, y as i32))
                            } else {
                                points
                            };
                            let updated_width = cmp::max(width, 1 + x as i32);
                            (updated_points, updated_width)
                        },
                    );
                    let updated_width = cmp::max(line_width, width);
                    let updated_height = cmp::max(1 + y as i32, height);
                    (updated_points, updated_width, updated_height)
                },
            );

        Tile {
            id,
            positions,
            width,
            height,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.positions.contains(&TilePosition(x as i32, y as i32)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            writeln!(f);
        }
        write!(f, "")
    }
}

#[derive(Clone)]
struct Image(Vector<Vector<bool>>);

struct ImageIterator {
    image: Image,
    rotated: i8,
    flipped: bool,
}

impl Iterator for ImageIterator {
    type Item = Image;

    fn next(&mut self) -> Option<Image> {
        if !self.flipped {
            if self.rotated <= 3 {
                println!("rotating");
                let result = Some(self.image.clone());
                self.image = self.image.rotate();
                self.rotated += 1;
                result
            } else {
                println!("flipping");
                let result = Some(self.image.clone());
                self.image = self.image.flip();
                self.rotated = 0;
                self.flipped = true;
                result
            }
        } else if self.rotated < 3 {
            println!("rotating");
            let result = Some(self.image.clone());
            self.image = self.image.rotate();
            self.rotated += 1;
            result
        } else {
            None
        }
    }
}

impl Image {
    fn render(&self) {
        print!("   ");
        for x in 0..self.width() {
            print!("{}", x / 10);
        }
        print!("\n   ");
        for x in 0..self.width() {
            print!("{}", x - 10 * (x / 10));
        }

        for y in 0..self.height() {
            print!("\n{:02} ", y);
            for x in 0..self.width() {
                print!("{}", if self.get(x, y) { '#' } else { '.' });
            }
        }
        println!();
    }

    fn render_with_pattern(
        &self,
        pattern: Vector<(usize, usize)>,
        positions: Vector<(usize, usize)>,
    ) {
        let monster_positions: HashSet<(usize, usize)> = positions
            .iter()
            .flat_map(|(x, y)| pattern.iter().map(move |(dx, dy)| (x + dx, y + dy)))
            .collect();

        print!("   ");
        for x in 0..self.width() {
            print!("{}", x / 10);
        }
        print!("\n   ");
        for x in 0..self.width() {
            print!("{}", x - 10 * (x / 10));
        }

        for y in 0..self.height() {
            print!("\n{:02} ", y);
            for x in 0..self.width() {
                let ch = if monster_positions.contains(&(x, y)) {
                    'O'
                } else if self.get(x, y) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", ch);
            }
        }
        println!();
    }

    fn count_pos_without_monster(
        &self,
        pattern: &Vector<(usize, usize)>,
        positions: &Vector<(usize, usize)>,
    ) -> usize {
        let monster_positions: HashSet<(usize, usize)> = positions
            .iter()
            .flat_map(|(x, y)| pattern.iter().map(move |(dx, dy)| (x + dx, y + dy)))
            .collect();

        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, &e)| e && !monster_positions.contains(&(*x, y)))
                    .count()
            })
            .sum()
    }

    fn variations(self) -> impl Iterator<Item = Image> {
        ImageIterator {
            image: self,
            rotated: 0,
            flipped: false,
        }
    }

    fn flip(&self) -> Image {
        let reversed: Vector<Vector<_>> = self
            .0
            .iter()
            .map(|row| row.iter().rev().copied().collect())
            .collect();
        Image(reversed)
    }

    fn rotate(&self) -> Image {
        let rotated = (0..self.width())
            .map(|y| {
                (0..self.height())
                    .map(|x| self.get(self.width() - y - 1, x))
                    .collect()
            })
            .collect();

        Image(rotated)
    }

    fn search(&self, pattern: &Vector<(usize, usize)>) -> Vector<(usize, usize)> {
        let pattern_height = *pattern.iter().map(|(_, y)| y).max().unwrap_or(&0);
        let pattern_width = *pattern.iter().map(|(x, _)| x).max().unwrap_or(&0);

        (0..self.height() - pattern_height)
            .flat_map(|y| (0..self.width() - pattern_width).map(move |x| (x, y)))
            .filter(|pos| self.check_pattern(pos, &pattern))
            .collect()
    }

    fn check_pattern(&self, position: &(usize, usize), pattern: &Vector<(usize, usize)>) -> bool {
        pattern
            .iter()
            .all(|(dx, dy)| self.get(position.0 + dx, position.1 + dy))
    }

    fn get(&self, x: usize, y: usize) -> bool {
        *get_in!( self.0, y =>  x ).unwrap()
    }

    fn width(&self) -> usize {
        self.0.get(0).map(|r| r.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn new(grid: &TileGrid) -> Image {
        let states: Vector<Vector<_>> = (0..grid.state_height())
            .map(|y| {
                (0..grid.state_width())
                    .map(|x| grid.state_at(x, y))
                    .collect()
            })
            .collect();

        Image(states)
    }
}
