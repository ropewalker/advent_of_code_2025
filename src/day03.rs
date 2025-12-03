use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(digit*));

    parser.parse(input).unwrap()
}

fn largest_joltage(bank: &[usize], num_batteries: usize) -> usize {
    let mut turned_on: Vec<usize> = bank[0..num_batteries].to_vec();

    for window in bank.windows(num_batteries).skip(1) {
        for i in 0..num_batteries {
            if window[i] > turned_on[i] {
                turned_on[i..].copy_from_slice(&window[i..]);
                break;
            }
        }
    }

    turned_on.iter().fold(0, |acc, x| acc * 10 + *x)
}

#[aoc(day3, part1)]
fn part1(banks: &[Vec<usize>]) -> usize {
    banks.iter().map(|bank| largest_joltage(bank, 2)).sum()
}

#[aoc(day3, part2)]
fn part2(banks: &[Vec<usize>]) -> usize {
    banks.iter().map(|bank| largest_joltage(bank, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 3_121_910_778_619);
    }
}
