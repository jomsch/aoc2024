use crate::util::day_runner;
use std::collections::HashMap;
use std::fmt::Display;

day_runner!("Day 1");

fn parse_line(line: &str) -> (usize, usize) {
    let mut split = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    (split.next().unwrap(), split.next().unwrap())
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines().map(parse_line);

    let (mut left, mut right) = lines.fold((Vec::new(), Vec::new()), |mut acc, val| {
        acc.0.push(val.0);
        acc.1.push(val.1);
        acc
    });

    left.sort();
    right.sort();

    (left, right)
}

pub fn part1(input: &str) -> impl Display {
    let (left, right) = parse_input(input);
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl Display {
    let (left, right) = parse_input(input);

    let similarity_cache: HashMap<usize, usize> =
        right.iter().fold(HashMap::new(), |mut hmap, val| {
            let count = hmap.get(val);
            hmap.insert(*val, count.unwrap_or(&0) + 1);
            hmap
        });

    left.iter()
        .map(|x| x * similarity_cache.get(x).unwrap_or(&0))
        .sum::<usize>()
}
