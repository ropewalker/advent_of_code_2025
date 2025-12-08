use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeMap, HashMap, HashSet};

type Position = (i64, i64, i64);

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Position> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(i64 "," i64 "," i64));

    parser.parse(input).unwrap()
}

fn distance_squared(position1: &Position, position2: &Position) -> i64 {
    (position2.0 - position1.0).pow(2)
        + (position2.1 - position1.1).pow(2)
        + (position2.2 - position1.2).pow(2)
}

fn three_largest_circuits_product(
    junction_box_positions: &[Position],
    total_connections: usize,
) -> usize {
    let mut positions_to_circuits: HashMap<Position, usize> = junction_box_positions
        .iter()
        .enumerate()
        .map(|(circuit_id, position)| (*position, circuit_id))
        .collect();

    let mut circuits: HashMap<usize, HashSet<Position>> = junction_box_positions
        .iter()
        .enumerate()
        .map(|(circuit_id, position)| (circuit_id, HashSet::from([*position])))
        .collect();

    let mut distances_squared_to_pairs: BTreeMap<i64, (Position, Position)> = BTreeMap::new();

    for i in 0..junction_box_positions.len() - 1 {
        for j in i + 1..junction_box_positions.len() {
            let distance_squared =
                distance_squared(&junction_box_positions[i], &junction_box_positions[j]);

            distances_squared_to_pairs.insert(
                distance_squared,
                (junction_box_positions[i], junction_box_positions[j]),
            );
        }
    }

    let mut num_connections = 0;

    for (position1, position2) in distances_squared_to_pairs.values() {
        let circuit1_id = *positions_to_circuits.get(position1).unwrap();
        let circuit2_id = *positions_to_circuits.get(position2).unwrap();

        if circuit1_id != circuit2_id {
            let circuit2 = circuits.remove(&circuit2_id).unwrap();

            for position in circuit2.iter() {
                positions_to_circuits.insert(*position, circuit1_id);
            }

            circuits.entry(circuit1_id).or_default().extend(circuit2);
        }

        num_connections += 1;

        if num_connections == total_connections {
            break;
        }
    }

    let mut circuit_sizes = circuits
        .values()
        .map(|circuit| circuit.len())
        .collect::<Vec<_>>();
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    circuit_sizes.iter().take(3).product()
}

#[aoc(day8, part1)]
fn part1(junction_box_positions: &[(i64, i64, i64)]) -> usize {
    three_largest_circuits_product(junction_box_positions, 1_000)
}

#[aoc(day8, part2)]
fn part2(junction_box_positions: &[(i64, i64, i64)]) -> i64 {
    let mut positions_to_circuits: HashMap<Position, usize> = junction_box_positions
        .iter()
        .enumerate()
        .map(|(circuit_id, position)| (*position, circuit_id))
        .collect();

    let mut circuits: HashMap<usize, HashSet<Position>> = junction_box_positions
        .iter()
        .enumerate()
        .map(|(circuit_id, position)| (circuit_id, HashSet::from([*position])))
        .collect();

    let mut distances_squared_to_pairs: BTreeMap<i64, (Position, Position)> = BTreeMap::new();

    for i in 0..junction_box_positions.len() - 1 {
        for j in i + 1..junction_box_positions.len() {
            let distance_squared =
                distance_squared(&junction_box_positions[i], &junction_box_positions[j]);

            distances_squared_to_pairs.insert(
                distance_squared,
                (junction_box_positions[i], junction_box_positions[j]),
            );
        }
    }

    for (position1, position2) in distances_squared_to_pairs.values() {
        let circuit1_id = *positions_to_circuits.get(position1).unwrap();
        let circuit2_id = *positions_to_circuits.get(position2).unwrap();

        if circuit1_id != circuit2_id {
            let circuit2 = circuits.remove(&circuit2_id).unwrap();

            for position in circuit2.iter() {
                positions_to_circuits.insert(*position, circuit1_id);
            }

            circuits.entry(circuit1_id).or_default().extend(circuit2);

            if circuits.len() == 1 {
                return position1.0 * position2.0;
            }
        }
    }

    let last_positions = distances_squared_to_pairs.values().last().unwrap();

    last_positions.0.0 * last_positions.1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn part1_example() {
        assert_eq!(
            three_largest_circuits_product(&parse_input(TEST_INPUT), 10),
            40
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 25_272);
    }
}
