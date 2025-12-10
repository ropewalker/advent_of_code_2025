use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

type Light = bool;
type Button = Vec<usize>;
type Joltage = usize;

struct Machine {
    lights: Vec<Light>,
    buttons: Vec<Button>,
    joltages: Vec<Joltage>,
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Machine> {
    use aoc_parse::{parser, prelude::*};

    let lights = parser!("[" {"#" => true, "." => false}* "]");
    let button = parser!("(" repeat_sep(usize, ",") ")");
    let buttons = parser!(repeat_sep(button, " "));
    let joltages = parser!("{" repeat_sep(usize, ",") "}");
    let parser = parser!(
        lines(
            lights:lights " " buttons:buttons " " joltages:joltages
                => Machine {
                    lights,
                    buttons,
                    joltages
                }
        )
    );

    parser.parse(input).unwrap()
}

fn configure_lights(machine: &Machine) -> usize {
    let mut queue = VecDeque::from([(vec![false; machine.lights.len()], 0)]);
    let mut visited = HashSet::from([vec![false; machine.lights.len()]]);

    while let Some((lights, presses)) = queue.pop_front() {
        if lights == machine.lights {
            return presses;
        }

        for button in machine.buttons.iter() {
            let mut new_lights = lights.clone();
            button
                .iter()
                .for_each(|index| new_lights[*index] = !new_lights[*index]);

            if !visited.contains(&new_lights) {
                visited.insert(new_lights.clone());
                queue.push_back((new_lights, presses + 1));
            }
        }
    }

    unreachable!()
}

fn configure_joltage_level_counters(machine: &Machine) -> usize {
    let mut queue = VecDeque::from([(
        vec![0; machine.joltages.len()],
        vec![0; machine.buttons.len()],
    )]);
    let mut visited = HashSet::from([vec![0; machine.buttons.len()]]);

    while let Some((joltages, presses)) = queue.pop_front() {
        if joltages == machine.joltages {
            return presses.iter().sum();
        }

        for (index, button) in machine.buttons.iter().enumerate() {
            let mut new_presses = presses.clone();
            new_presses[index] += 1;

            let mut new_joltages = joltages.clone();
            button.iter().for_each(|index| new_joltages[*index] += 1);

            if !visited.contains(&new_presses)
                && !new_joltages
                    .iter()
                    .enumerate()
                    .any(|(index, joltage)| *joltage > machine.joltages[index])
            {
                visited.insert(new_presses.clone());
                queue.push_back((new_joltages, new_presses));
            }
        }
    }

    unreachable!()
}

#[aoc(day10, part1)]
fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(configure_lights).sum()
}

#[aoc(day10, part2)]
fn part2(machines: &[Machine]) -> usize {
    machines.iter().map(configure_joltage_level_counters).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 33);
    }
}
