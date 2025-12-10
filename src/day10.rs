use aoc_runner_derive::{aoc, aoc_generator};
use num_rational::Rational32;
use std::collections::{HashSet, VecDeque};

type Light = bool;
type Button = Vec<usize>;
type Joltage = i32;

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
    let joltages = parser!("{" repeat_sep(i32, ",") "}");
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

fn multiply(vector: &mut [Rational32], number: &Rational32) {
    vector.iter_mut().for_each(|x| *x *= number);
}

fn subtract(vector1: &mut [Rational32], vector2: &[Rational32]) {
    vector1
        .iter_mut()
        .enumerate()
        .for_each(|(i, element)| *element -= vector2[i]);
}

fn make_positive(vector: &mut [Rational32]) {
    if let Some(j) = vector.iter().position(|e| *e != Rational32::ZERO)
        && vector[j] < Rational32::ZERO
    {
        vector.iter_mut().for_each(|e| *e *= -Rational32::ONE);
    }
}

fn row_echelon_form(matrix: &mut [Vec<Rational32>]) {
    matrix.sort_unstable_by(|a, b| b.cmp(a));

    for i in 0..matrix.len() {
        let current_row = &matrix[i].clone();

        if let Some(j) = matrix[i].iter().position(|e| *e != Rational32::ZERO) {
            let leading_element = matrix[i][j];

            for (k, row) in matrix.iter_mut().enumerate().skip(i + 1) {
                if k != i && row[j] != Rational32::ZERO {
                    let matrix_k_j = row[j];

                    multiply(row, &(leading_element / matrix_k_j));
                    subtract(row, current_row);
                    make_positive(row);
                }
            }

            matrix.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
}

fn solve(matrix: &[Vec<Rational32>], variable_values: &[Rational32]) -> Vec<Rational32> {
    let mut variable_values = variable_values.to_owned();

    for row in matrix.iter().rev() {
        if let Some(j) = row.iter().position(|e| *e != Rational32::ZERO) {
            variable_values[j] = (row[row.len() - 1]
                - row
                    .iter()
                    .enumerate()
                    .take(row.len() - 1)
                    .skip(j)
                    .map(|(k, element)| *element * variable_values[k])
                    .sum::<Rational32>())
                / row[j];
        }
    }

    variable_values
}

fn configure_joltage_level_counters(machine: &Machine) -> i32 {
    let mut coefficients: Vec<Vec<Rational32>> =
        vec![vec![Rational32::from_integer(0); machine.buttons.len()]; machine.joltages.len()];

    machine
        .buttons
        .iter()
        .enumerate()
        .for_each(|(button_index, button)| {
            button.iter().for_each(|joltage_level_counter| {
                coefficients[*joltage_level_counter][button_index] = Rational32::from_integer(1);
            })
        });

    machine
        .joltages
        .iter()
        .enumerate()
        .for_each(|(i, joltage)| coefficients[i].push(Rational32::from_integer(*joltage)));

    row_echelon_form(&mut coefficients);

    let mut free_indexes: HashSet<usize> = (0..machine.buttons.len()).collect();

    for row in &coefficients {
        if let Some(j) = row.iter().position(|e| *e != Rational32::ZERO) {
            free_indexes.remove(&j);
        }
    }

    let mut states = VecDeque::from([vec![Rational32::ZERO; machine.buttons.len()]]);
    let mut min_total_presses = machine.joltages.iter().sum::<i32>();
    let max_button_presses = Rational32::from_integer(*machine.joltages.iter().max().unwrap());
    let mut visited = HashSet::from([vec![Rational32::ZERO; machine.buttons.len()]]);

    while let Some(state) = states.pop_front() {
        let result = solve(&coefficients, &state);

        if result.iter().all(|button_presses| {
            button_presses.is_integer() && *button_presses >= Rational32::ZERO
        }) {
            let presses: i32 = result.iter().sum::<Rational32>().to_integer();

            if min_total_presses > presses {
                min_total_presses = presses;
            }
        }

        if result
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if free_indexes.contains(&index) {
                    Some(*value)
                } else {
                    None
                }
            })
            .sum::<Rational32>()
            > Rational32::from_integer(min_total_presses)
        {
            continue;
        }

        for index in free_indexes.iter() {
            let mut new_state = state.clone();
            new_state[*index] += 1;

            if !visited.contains(&new_state) && new_state[*index] <= max_button_presses {
                states.push_back(new_state.clone());
                visited.insert(new_state);
            }
        }
    }

    min_total_presses
}

#[aoc(day10, part1)]
fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(configure_lights).sum()
}

#[aoc(day10, part2)]
fn part2(machines: &[Machine]) -> i32 {
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
