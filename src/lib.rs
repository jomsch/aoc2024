pub mod day1;

mod util {
    /// Generates a run function which runs part1 and part2 of the day
    macro_rules! day_runner {
        ( $x:expr) => {
            pub fn run(input: &str) {
                println!("$x",);
                let result_part1 = part1(input);
                println!("\t Part 1 Result: {result_part1}");

                let result_part2 = part2(input);
                println!("\t Part 2 Result: {result_part2}");
            }
        };
    }

    pub(crate) use day_runner;
}
