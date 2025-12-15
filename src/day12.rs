use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Eq, PartialEq)]
struct Shape(Vec<Vec<bool>>);

#[derive(Clone, Eq, PartialEq)]
struct RegionPacking {
    width: usize,
    height: usize,
    shapes_to_fit: Vec<usize>,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> (Vec<Shape>, Vec<RegionPacking>) {
    use aoc_parse::{parser, prelude::*};

    let shape_index = parser!(line(usize ":"));
    let shape_parts = parser!(lines({"#" => true, "." => false}*));
    let shape = parser!(section(_index:shape_index parts:shape_parts => Shape(parts)));

    let region = parser!(
        line(width:usize "x" height:usize ": " shapes_to_fit:repeat_sep(usize, " ")
            => RegionPacking {width, height, shapes_to_fit})
    );

    let parser = parser!(shape* section(region*));

    parser.parse(input).unwrap()
}

#[aoc(day12, part1)]
fn part1((shapes, regions): &(Vec<Shape>, Vec<RegionPacking>)) -> usize {
    let shape_sizes = shapes
        .iter()
        .map(|shape| {
            shape
                .0
                .iter()
                .map(|row| row.iter().filter(|x| **x).count())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    regions
        .iter()
        .filter(|region_packing| {
            region_packing.height * region_packing.width
                >= region_packing
                    .shapes_to_fit
                    .iter()
                    .enumerate()
                    .map(|(shape_index, shape_count)| shape_sizes[shape_index] * shape_count)
                    .sum()
        })
        .count()
}
