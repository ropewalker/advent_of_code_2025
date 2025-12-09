use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<(i64, i64)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(i64 "," i64));

    parser.parse(input).unwrap()
}

#[aoc(day9, part1)]
fn part1(red_tiles: &[(i64, i64)]) -> i64 {
    let mut largest_area = 0;

    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            let area = ((red_tiles[i].0 - red_tiles[j].0).abs() + 1)
                * ((red_tiles[i].1 - red_tiles[j].1).abs() + 1);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}

#[aoc(day9, part2)]
fn part2(red_tiles: &[(i64, i64)]) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 24);
    }
}
