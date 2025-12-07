use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut starting_point = (0, 0);
    let mut splitters = HashSet::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, character)| match character {
                'S' => {
                    starting_point = (x as i32, y as i32);
                }
                '^' => {
                    splitters.insert((x as i32, y as i32));
                }
                _ => {}
            })
    });

    (starting_point, splitters)
}

#[aoc(day7, part1)]
fn part1((staring_point, splitters): &((i32, i32), HashSet<(i32, i32)>)) -> usize {
    let lowest_ordinate = *splitters.iter().map(|(_, y)| y).max().unwrap();

    let mut queue = VecDeque::from([*staring_point]);
    let mut visited_splitters = HashSet::new();

    while let Some(point) = queue.pop_front() {
        let next_point = (point.0, point.1 + 1);

        if next_point.1 > lowest_ordinate {
            break;
        }

        if splitters.contains(&next_point) {
            if !visited_splitters.contains(&next_point) {
                queue.push_back((next_point.0 - 1, next_point.1));
                queue.push_back((next_point.0 + 1, next_point.1));

                visited_splitters.insert(next_point);
            }
        } else {
            queue.push_back(next_point);
        }
    }

    visited_splitters.len()
}

#[aoc(day7, part2)]
fn part2((staring_point, splitters): &((i32, i32), HashSet<(i32, i32)>)) -> usize {
    let lowest_ordinate = *splitters.iter().map(|(_, y)| y).max().unwrap();

    let mut queue = VecDeque::from([(*staring_point, *staring_point)]);
    let mut visited_splitters = HashMap::from([(*staring_point, 1)]);

    let mut timeline_count = 0;

    while let Some((point, prev_splitter)) = queue.pop_front() {
        let prev_timeline_count = visited_splitters.get(&prev_splitter).unwrap();
        let next_point = (point.0, point.1 + 1);

        if next_point.1 > lowest_ordinate {
            timeline_count += prev_timeline_count;
            continue;
        }

        if splitters.contains(&next_point) {
            if let Some(timeline_count) = visited_splitters.get(&next_point) {
                visited_splitters.insert(next_point, timeline_count + prev_timeline_count);
            } else {
                queue.push_back(((next_point.0 - 1, next_point.1), next_point));
                queue.push_back(((next_point.0 + 1, next_point.1), next_point));

                visited_splitters.insert(next_point, *prev_timeline_count);
            }
        } else {
            queue.push_back((next_point, prev_splitter));
        }
    }

    timeline_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 40);
    }
}
