use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    RollOfPaper,
    Empty,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    use Tile::*;
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines({"@" => RollOfPaper, "." => Empty}*));

    parser.parse(input).unwrap()
}

fn accessible_rolls(grid: &[Vec<Tile>]) -> HashSet<(i32, i32)> {
    use Tile::*;

    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;

    let mut accessible_rolls = HashSet::new();

    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] == RollOfPaper {
                let mut adjacent_rolls = 0;

                for adjacent_y in i32::max(0, y - 1)..=i32::min(max_y - 1, y + 1) {
                    for adjacent_x in i32::max(0, x - 1)..=i32::min(max_x - 1, x + 1) {
                        if (adjacent_x != x || adjacent_y != y)
                            && grid[adjacent_y as usize][adjacent_x as usize] == RollOfPaper
                        {
                            adjacent_rolls += 1;
                        }
                    }
                }

                if adjacent_rolls < 4 {
                    accessible_rolls.insert((x, y));
                }
            }
        }
    }

    accessible_rolls
}

#[aoc(day4, part1)]
fn part1(grid: &[Vec<Tile>]) -> usize {
    accessible_rolls(grid).len()
}

#[aoc(day4, part2)]
fn part2(grid: &[Vec<Tile>]) -> usize {
    use Tile::*;

    let mut grid = grid.to_owned();
    let mut accessible_rolls = accessible_rolls(&grid);
    let mut removed = 0;

    while !accessible_rolls.is_empty() {
        accessible_rolls
            .iter()
            .for_each(|(x, y)| grid[*y as usize][*x as usize] = Empty);

        removed += accessible_rolls.len();
        accessible_rolls = crate::day04::accessible_rolls(&grid);
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 43);
    }
}
