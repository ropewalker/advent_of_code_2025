use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

type Point = (i64, i64);
type Edge = (Point, Point);

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Point> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(i64 "," i64));

    parser.parse(input).unwrap()
}

fn rectangle_area(tile1: &Point, tile2: &Point) -> i64 {
    ((tile1.0 - tile2.0).abs() + 1) * ((tile1.1 - tile2.1).abs() + 1)
}

#[aoc(day9, part1)]
fn part1(red_tiles: &[Point]) -> i64 {
    let mut largest_area = 0;

    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            let area = rectangle_area(&red_tiles[i], &red_tiles[j]);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}

fn is_point_on_polygon_edge(point: &Point, edge: &Edge) -> bool {
    if edge.0.0 == edge.1.0
        && point.0 == edge.0.0
        && point.1 >= i64::min(edge.0.1, edge.1.1)
        && point.1 <= i64::max(edge.0.1, edge.1.1)
    {
        return true;
    }

    if edge.0.1 == edge.1.1
        && point.1 == edge.0.1
        && point.0 >= i64::min(edge.0.0, edge.1.0)
        && point.0 <= i64::max(edge.0.0, edge.1.0)
    {
        return true;
    }

    false
}

fn is_point_inside_polygon(
    point: &Point,
    polygon_edges: &[Edge],
    polygon_vertices: &BTreeSet<Point>,
) -> bool {
    if polygon_vertices.contains(point) {
        return true;
    }

    let mut intersection_count = 0;

    for edge in polygon_edges.iter() {
        if is_point_on_polygon_edge(point, edge) {
            return true;
        }

        if edge.0.0 == edge.1.0
            && point.0 <= edge.0.0
            && point.1 > i64::min(edge.0.1, edge.1.1)
            && point.1 <= i64::max(edge.0.1, edge.1.1)
        {
            intersection_count += 1;
        }
    }

    intersection_count % 2 == 1
}

#[aoc(day9, part2)]
fn part2(red_tiles: &[(i64, i64)]) -> i64 {
    let polygon_edges = red_tiles
        .iter()
        .copied()
        .cycle()
        .zip(red_tiles.iter().copied().cycle().skip(1))
        .take(red_tiles.len())
        .collect::<Vec<_>>();

    let red_tiles = red_tiles.iter().copied().collect::<BTreeSet<_>>();

    let mut largest_area = 0;

    for (left_x, left_y) in red_tiles.iter() {
        'next_tile: for (right_x, right_y) in
            red_tiles.range((Excluded((*left_x, *left_y)), Unbounded))
        {
            if !is_point_inside_polygon(&(*left_x, *left_y), &polygon_edges, &red_tiles)
                || !is_point_inside_polygon(&(*left_x, *right_y), &polygon_edges, &red_tiles)
                || !is_point_inside_polygon(&(*right_x, *left_y), &polygon_edges, &red_tiles)
                || !is_point_inside_polygon(&(*right_x, *right_y), &polygon_edges, &red_tiles)
            {
                continue 'next_tile;
            }

            for edge in polygon_edges.iter() {
                if edge.0.0 == edge.1.0
                    && *left_x < edge.0.0
                    && *right_x > edge.0.0
                    && (*left_y > i64::min(edge.0.1, edge.1.1)
                        && *left_y < i64::max(edge.0.1, edge.1.1)
                        || *right_y > i64::min(edge.0.1, edge.1.1)
                            && *right_y < i64::max(edge.0.1, edge.1.1))
                {
                    continue 'next_tile;
                }

                if edge.0.1 == edge.1.1
                    && i64::min(*left_y, *right_y) < edge.0.1
                    && i64::max(*left_y, *right_y) > edge.0.1
                    && (*left_x > i64::min(edge.0.0, edge.1.0)
                        && *left_x < i64::max(edge.0.0, edge.1.0)
                        || *right_x > i64::min(edge.0.0, edge.1.0)
                            && *right_x < i64::max(edge.0.0, edge.1.0))
                {
                    continue 'next_tile;
                }
            }

            let area = rectangle_area(&(*left_x, *left_y), &(*right_x, *right_y));

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 24);
    }
}
