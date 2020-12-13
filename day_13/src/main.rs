extern crate im_rc;

use im_rc::Vector;

fn main() {
    println!("--- [AoC 2020] Day 13: Shuttle Search ---");
    let input = utils::read_strings_from_param();
    println!("Solution to part one: {}", part_one(&input).unwrap());
    println!("Solution to part two: {}", part_two(&input).unwrap());
}

fn part_one(input: &Vector<String>) -> Option<usize> {
    let departure_time = input.head().unwrap().parse::<usize>().unwrap();
    let busses = input
        .last()
        .unwrap()
        .split(',')
        .filter(|&a| a != "x")
        .map(|w| w.parse::<usize>().unwrap());

    busses
        .map(|bus_id| (bus_id, calculate_wait_time(&departure_time, &bus_id)))
        .min_by_key(|(_, wait)| *wait)
        .map(|(bus_id, wait)| bus_id * wait)
}

fn part_two(input: &Vector<String>) -> Option<usize> {
    let busses: Vector<_> = input
        .last()
        .unwrap()
        .split(',')
        .map(|l| {
            Some(l)
                .filter(|&b| b != "x")
                .map(|i| i.parse::<usize>().unwrap())
        })
        .enumerate()
        .filter_map(|(offset, bus)| bus.map(|id| (id, offset)))
        .collect();

    let (solution, _) = busses.iter().fold((0, 1), |(t, prod), (bus_id, offset)| {
        let next_timestamp = std::iter::successors(Some(t), |i| Some(i + prod))
            .find(|t| (t + offset) % bus_id == 0)
            .unwrap();
        (next_timestamp, prod * bus_id)
    });
    Some(solution)
}

fn calculate_wait_time(departure_time: &usize, loop_duration: &usize) -> usize {
    loop_duration - departure_time.rem_euclid(*loop_duration)
}

#[cfg(test)]
mod test {

    use super::*;
    use im_rc::vector;

    #[test]
    fn test_part_two() {
        let input = vector!(String::from("7,11"));
        assert_eq!(part_two(&input), Some(21));

        let input = vector!(String::from("17,x,13,19"));
        assert_eq!(part_two(&input), Some(3417));

        let input = vector!(String::from("67,7,59,61"));
        assert_eq!(part_two(&input), Some(754018));

        let input = vector!(String::from("67,x,7,59,61"));
        assert_eq!(part_two(&input), Some(779210));

        let input = vector!(String::from("67,7,x,59,61"));
        assert_eq!(part_two(&input), Some(1261476));

        let input = vector!(String::from("1789,37,47,1889"));
        assert_eq!(part_two(&input), Some(1202161486));

        let input = vector!(String::from("7,13,x,x,59,x,31,19"));
        assert_eq!(part_two(&input), Some(1068781));
    }
}
