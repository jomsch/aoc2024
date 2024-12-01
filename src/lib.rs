use std::fmt::Display;

mod day1;

pub use day1::Day1;

pub trait Solution {
    const NAME: &'static str;

    fn run(input: &str) {
        println!("{}", Self::NAME);
        let result_part1 = Self::part1(input);
        println!("\t Part 1 Result: {result_part1}");

        let result_part2 = Self::part2(input);
        println!("\t Part 2 Result: {result_part2}");
    }

    fn part1(input: &str) -> impl Display;
    fn part2(input: &str) -> impl Display;
}
