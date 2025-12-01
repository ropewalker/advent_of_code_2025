use aoc_runner_derive::{aoc, aoc_generator};

const STARTING_POSITION: i32 = 50;
const DIALS_COUNT: i32 = 100;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines({
        "L" distance:i32 => -distance,
        "R" distance:i32 => distance
    }));
    parser.parse(input).unwrap()
}

#[aoc(day1, part1)]
fn part1(rotations: &[i32]) -> usize {
    let mut result = 0;

    rotations
        .iter()
        .fold(STARTING_POSITION, |position, rotation| {
            let new_position = (position + rotation) % DIALS_COUNT;

            if new_position == 0 {
                result += 1;
            };

            new_position
        });

    result
}

#[aoc(day1, part2)]
fn part2(rotations: &[i32]) -> usize {
    let mut result = 0;

    rotations
        .iter()
        .fold(STARTING_POSITION, |position, rotation| {
            let new_position = position + rotation;

            result += (new_position / DIALS_COUNT).unsigned_abs() as usize;

            if new_position <= 0 && position != 0 {
                result += 1;
            }

            (new_position % DIALS_COUNT + DIALS_COUNT) % DIALS_COUNT
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 3);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 6);
    }
}
