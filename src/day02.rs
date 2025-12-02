use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<(u128, u128)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(repeat_sep(u128 "-" u128, ","));

    parser.parse(input).unwrap()
}

fn find_invalid_ids(first_id: u128, last_id: u128) -> Vec<u128> {
    let mut result = Vec::new();

    let first_id_digits = first_id.ilog10() + 1;
    let last_id_digits = last_id.ilog10() + 1;

    let half_min_digits = if first_id_digits.is_multiple_of(2) {
        u32::max(first_id_digits / 2, 1)
    } else {
        first_id_digits / 2 + 1
    };
    let half_max_digits = last_id_digits / 2;

    for digits in half_min_digits..=half_max_digits {
        for half in u128::max(10u128.pow(digits - 1), first_id / 10u128.pow(digits))
            ..=u128::min(10u128.pow(digits) - 1, last_id / 10u128.pow(digits))
        {
            let id = half * 10u128.pow(digits) + half;

            if id >= first_id && id <= last_id {
                result.push(id);
            }
        }
    }

    result
}

#[aoc(day2, part1)]
fn part1(pairs: &[(u128, u128)]) -> u128 {
    pairs
        .iter()
        .flat_map(|(first_id, last_id)| find_invalid_ids(*first_id, *last_id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1_227_775_554);
    }
}
