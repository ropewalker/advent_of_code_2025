use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

const STARTING_DEVICE: &str = "you";
const OUTPUT_DEVICE: &str = "out";
const DAC: &str = "dac";
const FFT: &str = "fft";
const SVR: &str = "svr";

#[aoc_generator(day11)]
fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(hash_map(lines(
        string(alpha+) ": " hash_set(repeat_sep(string(alpha+), " "))
    )));

    parser.parse(input).unwrap()
}

#[aoc(day11, part1)]
fn part1(connections: &HashMap<String, HashSet<String>>) -> usize {
    let mut paths_count = 0;
    let mut queue = Vec::from([STARTING_DEVICE.to_string()]);

    while let Some(device) = queue.pop() {
        if device == OUTPUT_DEVICE {
            paths_count += 1;
        } else if let Some(connected_devices) = connections.get(&device) {
            connected_devices
                .iter()
                .for_each(|connected_device| queue.push(connected_device.to_owned()));
        }
    }

    paths_count
}

fn find_paths(
    start: &str,
    connections: &HashMap<String, HashSet<String>>,
    cache: &mut HashMap<String, (usize, usize, usize, usize)>,
) -> (usize, usize, usize, usize) {
    if start == OUTPUT_DEVICE {
        return (0, 0, 0, 1);
    }

    if let Some(result) = cache.get(start) {
        return *result;
    }

    let (mut dac_paths, mut fft_paths, mut dac_and_fft_paths, mut total_paths) = (0, 0, 0, 0);

    if let Some(connected_devices) = connections.get(start) {
        for connected_device in connected_devices {
            let (child_dac_paths, child_fft_paths, child_dac_and_fft_paths, child_total_paths) =
                find_paths(connected_device, connections, cache);

            dac_paths += if start == DAC {
                child_total_paths
            } else {
                child_dac_paths
            };
            fft_paths += if start == FFT {
                child_total_paths
            } else {
                child_fft_paths
            };

            dac_and_fft_paths += match start {
                DAC => child_fft_paths,
                FFT => child_dac_paths,
                _ => child_dac_and_fft_paths,
            };

            total_paths += child_total_paths;
        }
    }

    cache.insert(
        start.to_owned(),
        (dac_paths, fft_paths, dac_and_fft_paths, total_paths),
    );

    (dac_paths, fft_paths, dac_and_fft_paths, total_paths)
}

#[aoc(day11, part2)]
fn part2(connections: &HashMap<String, HashSet<String>>) -> usize {
    find_paths(SVR, connections, &mut HashMap::new()).2
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    static TEST_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 2);
    }
}
