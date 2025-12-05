use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        section(lines(usize "-" usize))
        section(lines(usize))
    );

    parser.parse(input).unwrap()
}

#[aoc(day5, part1)]
fn part1(
    (fresh_ingredient_id_ranges, available_ingredient_ids): &(Vec<(usize, usize)>, Vec<usize>),
) -> usize {
    available_ingredient_ids
        .iter()
        .filter(|id| {
            fresh_ingredient_id_ranges
                .iter()
                .any(|(start, finish)| start <= id && finish >= id)
        })
        .count()
}

#[aoc(day5, part2)]
fn part2((fresh_ingredient_id_ranges, _): &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let all_border_ids: BTreeSet<usize> = fresh_ingredient_id_ranges
        .iter()
        .flat_map(|(start, finish)| [*start, *finish])
        .collect();

    let mut total_id_count = 0;
    let mut starting_id = *all_border_ids.iter().next().unwrap();

    let mut ranges_count = fresh_ingredient_id_ranges
        .iter()
        .filter(|(start, _)| *start == starting_id)
        .count();

    for next_id in all_border_ids.iter().skip(1) {
        if ranges_count == 0 {
            starting_id = *next_id;
        }

        ranges_count = ranges_count
            + fresh_ingredient_id_ranges
                .iter()
                .filter(|(start, _)| *start == *next_id)
                .count()
            - fresh_ingredient_id_ranges
                .iter()
                .filter(|(_, finish)| *finish == *next_id)
                .count();

        if ranges_count == 0 {
            total_id_count += next_id - starting_id + 1;
            starting_id = *next_id;
        }
    }

    total_id_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 14);
    }
}
