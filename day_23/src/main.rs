extern crate im_rc;
extern crate log;
extern crate pretty_env_logger;

use im_rc::vector;
use im_rc::HashMap;
use im_rc::Vector;
use itertools::Itertools;
use log::{debug, info};

fn main() {
    pretty_env_logger::init();
    info!("--- [AoC 2020] Day 23: Crab Cups ---");

    let input = vector![9, 1, 6, 4, 3, 8, 2, 7, 5];

    info!("Solution to part one: {}", part_one(&input));
}

fn part_one(input: &Vector<u32>) -> String {
    let ring = Ring::new(&input);
    let largest = input.iter().max().unwrap();
    let active = *input.head().unwrap();

    let (_, mut ring) = (1..=100).fold((active, ring), |(active, ring), nb_move| {
        do_move(&nb_move, active, largest, ring)
    });

    ring.pick_up_after(1, 8).iter().join("")
}

fn do_move(move_nb: &usize, active: u32, largest: &u32, mut ring: Ring) -> (u32, Ring) {
    debug!("-- move {} --", move_nb);
    debug!("cups: {:?}", ring.values());
    let cups = ring.pick_up_after(active, 3);
    debug!("pick up: {:?}", cups);

    let destination = (1..=4)
        .map(|d| {
            if (active as i64) - d <= 0 {
                *largest - d as u32 + active
            } else {
                active - d as u32
            }
        })
        .find(|dest| !cups.contains(dest))
        .unwrap();
    debug!("destination: {}", destination);

    ring.insert_after(destination, cups);
    let active = *ring.next(&active);
    (active, ring)
}

struct Ring {
    forward_links: HashMap<u32, u32>,
}

impl Ring {
    fn len(&self) -> usize {
        self.forward_links.len()
    }

    fn values(&self) -> Vector<&u32> {
        if let Some(start) = self.forward_links.keys().next() {
            let mut result = vector!(start);
            let mut next = self.next(start);
            while next != start {
                result.push_back(next);
                next = self.next(next);
            }
            result
        } else {
            Vector::new()
        }
    }

    fn next(&self, elem: &u32) -> &u32 {
        self.forward_links.get(elem).unwrap()
    }

    fn pick_up_after(&mut self, start: u32, length: usize) -> Vector<u32> {
        let (end, slice) = (0..length).fold((start, vector!()), |(prev, mut slice), _| {
            let next = self.forward_links.remove(&prev).unwrap();
            slice.push_back(next);
            (next, slice)
        });

        let tail = self.forward_links.remove(&end).unwrap();
        self.forward_links.insert(start, tail);
        slice
    }

    fn insert_after(&mut self, start: u32, elements: Vector<u32>) {
        let tail = self.forward_links.remove(&start).unwrap();
        let last = elements.iter().fold(start, |prev, &next| {
            self.forward_links.insert(prev, next);
            next
        });

        self.forward_links.insert(last, tail);
    }

    fn new(inputs: &Vector<u32>) -> Ring {
        match inputs.head() {
            Some(&head) => {
                let (mut links, last) = inputs.iter().skip(1).copied().fold(
                    (HashMap::new(), head),
                    |(mut links, last), elem| {
                        links.insert(last, elem);
                        (links, elem)
                    },
                );
                links.insert(last, head);
                Ring {
                    forward_links: links,
                }
            }
            None => Ring {
                forward_links: HashMap::new(),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_ring_pick_up() {
        let input = vector!(1, 2, 3, 4, 5);

        let mut ring = Ring::new(&input);
        let slice = ring.pick_up_after(1, 3);
        assert_eq!(slice, vector!(2, 3, 4));

        let mut ring = Ring::new(&input);
        let slice = ring.pick_up_after(5, 2);
        assert_eq!(slice, vector!(1, 2));

        let mut ring = Ring::new(&input);
        let slice = ring.pick_up_after(4, 3);
        assert_eq!(slice, vector!(5, 1, 2));
    }

    #[test]
    fn test_ring_insert() {
        let input = vector!(1, 2, 3, 4);
        let mut ring = Ring::new(&input);

        ring.insert_after(2, vector!(5, 6));
        let slice = ring.pick_up_after(1, 4);
        assert_eq!(slice, vector!(2, 5, 6, 3));
    }
}
