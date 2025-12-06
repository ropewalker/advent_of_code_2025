use aoc_runner_derive::aoc;

use Operator::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Operator {
    Add,
    Mul,
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let mut operands: Vec<Vec<u64>> = Vec::with_capacity(input.lines().count() - 1);

    for line in input.lines().take(input.lines().count() - 1) {
        let mut row: Vec<u64> = Vec::new();
        let mut current_number = 0;
        let mut is_number = false;

        for character in line.chars() {
            match character {
                x if x.is_ascii_digit() => {
                    is_number = true;
                    current_number = 10 * current_number + x.to_digit(10).unwrap() as u64;
                }
                ' ' => {
                    if is_number {
                        row.push(current_number);
                        is_number = false;
                        current_number = 0;
                    }
                }
                _ => unreachable!(),
            }
        }

        if is_number {
            row.push(current_number);
        }

        operands.push(row);
    }

    let operators: Vec<Operator> =
        input
            .lines()
            .last()
            .unwrap()
            .chars()
            .fold(Vec::new(), |mut acc, x| {
                match x {
                    '+' => acc.push(Add),
                    '*' => acc.push(Mul),
                    _ => {}
                };
                acc
            });

    operators
        .iter()
        .enumerate()
        .map(|(index, operator)| {
            (1..operands.len()).fold(operands[0][index], |acc, y| match operator {
                Add => acc + operands[y][index],
                Mul => acc * operands[y][index],
            })
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let characters: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut buffer: Vec<u64> = Vec::new();
    let mut result = 0;

    'next_column: for x in (0..characters[0].len()).rev() {
        let mut is_number = false;
        let mut current_number = 0;

        for row in &characters {
            match row[x] {
                x if x.is_ascii_digit() => {
                    is_number = true;
                    current_number = 10 * current_number + x.to_digit(10).unwrap() as u64;
                }
                '+' => {
                    buffer.push(current_number);

                    result += buffer.iter().sum::<u64>();
                    buffer.clear();
                    continue 'next_column;
                }
                '*' => {
                    buffer.push(current_number);

                    result += buffer.iter().product::<u64>();
                    buffer.clear();
                    continue 'next_column;
                }
                _ => {}
            }
        }

        if is_number {
            buffer.push(current_number);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT), 4_277_556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), 3_263_827);
    }
}
