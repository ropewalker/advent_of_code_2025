use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> () {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!("");

    parser.parse(input).unwrap()
}

#[aoc(day1, part1)]
fn part1(input: &()) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_1: &str = "";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 0);
    }
}
