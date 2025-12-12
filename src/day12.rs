use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Shape(Vec<Vec<bool>>);

impl Debug for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for part in row.iter() {
                if *part {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
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

impl Shape {
    fn rotate(&self) -> Shape {
        let mut new_shape: Shape = Shape(vec![vec![false; self.0[0].len()]; self.0.len()]);

        (0..self.0[0].len()).for_each(|x| {
            (0..self.0.len()).for_each(|y| new_shape.0[y][x] = self.0[self.0.len() - 1 - x][y])
        });

        new_shape
    }

    fn flip(&self) -> Shape {
        let mut new_shape: Shape = Shape(vec![vec![false; self.0[0].len()]; self.0.len()]);

        (0..self.0.len()).for_each(|x| {
            (0..self.0[0].len()).for_each(|y| new_shape.0[y][x] = self.0[y][self.0.len() - 1 - x])
        });

        new_shape
    }

    fn transformations(&self) -> HashSet<Shape> {
        let mut transformations = HashSet::new();

        let mut new_shape = self.clone();

        for _ in 0..4 {
            new_shape = new_shape.rotate();
            transformations.insert(new_shape.clone());
        }

        new_shape = new_shape.flip();

        for _ in 0..4 {
            new_shape = new_shape.rotate();
            transformations.insert(new_shape.clone());
        }

        transformations
    }
}

fn pack_shape(region: &[Vec<bool>], shape: &Shape) -> HashSet<Vec<Vec<bool>>> {
    let mut new_regions = HashSet::new();

    for corner_y in 0..region.len() - shape.0.len() + 1 {
        'next_corner: for corner_x in 0..region[corner_y].len() - shape.0[corner_y].len() + 1 {
            let mut new_region = region.to_owned();

            for (y, row) in new_region
                .iter_mut()
                .enumerate()
                .skip(corner_y)
                .take(shape.0.len())
            {
                for (x, tile) in row
                    .iter_mut()
                    .enumerate()
                    .skip(corner_x)
                    .take(shape.0[0].len())
                {
                    if *tile && shape.0[y - corner_y][x - corner_x] {
                        continue 'next_corner;
                    } else {
                        *tile = *tile || shape.0[y - corner_y][x - corner_x];
                    }
                }
            }

            new_regions.insert(new_region);
        }
    }

    new_regions
}

fn pack_shapes(
    region: &[Vec<bool>],
    shapes_to_fit: &[usize],
    shapes: &[Shape],
    cache: &mut HashMap<(Vec<Vec<bool>>, Vec<usize>), bool>,
) -> bool {
    if let Some(result) = cache.get(&(region.to_owned(), shapes_to_fit.to_owned())) {
        return *result;
    }

    if let Some(shape_index) = shapes_to_fit.iter().position(|number| *number > 0) {
        let mut remaining_shapes_to_pack = shapes_to_fit.to_owned();
        remaining_shapes_to_pack[shape_index] -= 1;

        for shape_variant in shapes[shape_index].transformations() {
            for updated_region in pack_shape(region, &shape_variant) {
                if pack_shapes(&updated_region, &remaining_shapes_to_pack, shapes, cache) {
                    cache.insert((region.to_owned(), shapes_to_fit.to_owned()), true);
                    return true;
                }
            }
        }

        cache.insert((region.to_owned(), shapes_to_fit.to_owned()), false);
        false
    } else {
        true
    }
}

#[aoc(day12, part1)]
fn part1((shapes, regions): &(Vec<Shape>, Vec<RegionPacking>)) -> usize {
    regions
        .iter()
        .filter_map(|region_packing| {
            if pack_shapes(
                &vec![vec![false; region_packing.width]; region_packing.height],
                &region_packing.shapes_to_fit,
                shapes,
                &mut HashMap::new(),
            ) {
                Some(1)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2);
    }
}
