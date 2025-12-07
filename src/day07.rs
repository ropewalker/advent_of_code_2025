use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut row = vec![false; input.lines().next().unwrap().len()];
    let mut split_count = 0;

    for line in input.lines().take(input.lines().count() - 1) {
        for (index, character) in line.chars().enumerate() {
            if character == '^' {
                if row[index] {
                    split_count += 1;
                }

                row[index] = false;
                row[index - 1] = true;
                row[index + 1] = true;
            } else if character == 'S' {
                row[index] = true;
            }
        }
    }

    split_count
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let first_line = input.lines().next().unwrap();
    let mut row = vec![1; first_line.len()];

    for line in input.lines().rev() {
        for (index, character) in line.chars().enumerate() {
            if character == '^' {
                row[index] = row[index - 1] + row[index + 1];
            }
        }
    }

    row[first_line.find('S').unwrap()]
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
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), 40);
    }
}
