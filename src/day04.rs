use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

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

fn count_neighbours(grid: &[Vec<Tile>]) -> Vec<Vec<Option<usize>>> {
    use Tile::*;

    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| {
                    if *tile == RollOfPaper {
                        let mut adjacent_rolls = 0;

                        for adjacent_y in
                            i32::max(0, y as i32 - 1)..=i32::min(max_y - 1, y as i32 + 1)
                        {
                            for adjacent_x in
                                i32::max(0, x as i32 - 1)..=i32::min(max_x - 1, x as i32 + 1)
                            {
                                if (adjacent_x != x as i32 || adjacent_y != y as i32)
                                    && grid[adjacent_y as usize][adjacent_x as usize] == RollOfPaper
                                {
                                    adjacent_rolls += 1;
                                }
                            }
                        }

                        Some(adjacent_rolls)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(grid: &[Vec<Tile>]) -> usize {
    count_neighbours(grid)
        .iter()
        .flatten()
        .filter(|tile| tile.is_some() && tile.unwrap() < 4)
        .count()
}

#[aoc(day4, part2)]
fn part2(grid: &[Vec<Tile>]) -> usize {
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;

    let mut neighbours_grid = count_neighbours(grid);
    let mut queue: VecDeque<(i32, i32)> = neighbours_grid
        .iter_mut()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter_mut().enumerate().filter_map(move |(x, tile)| {
                if tile.is_some() && tile.unwrap() < 4 {
                    *tile = None;
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut removed = queue.len();

    while let Some((x, y)) = queue.pop_front() {
        for adjacent_y in i32::max(0, y - 1)..=i32::min(max_y - 1, y + 1) {
            for adjacent_x in i32::max(0, x - 1)..=i32::min(max_x - 1, x + 1) {
                if (adjacent_x != x || adjacent_y != y)
                    && let Some(neighbour_count) =
                        neighbours_grid[adjacent_y as usize][adjacent_x as usize]
                {
                    let new_neighbour_count = neighbour_count - 1;

                    if new_neighbour_count < 4 {
                        queue.push_back((adjacent_x, adjacent_y));
                        neighbours_grid[adjacent_y as usize][adjacent_x as usize] = None;
                        removed += 1;
                    } else {
                        neighbours_grid[adjacent_y as usize][adjacent_x as usize] =
                            Some(new_neighbour_count);
                    }
                }
            }
        }
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
