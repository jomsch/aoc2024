use aoc2024::Solution;

fn main() {
    let args = Args::from_env();

    let input = std::fs::read_to_string(&args.input_path).unwrap();

    match args.day {
        1 => aoc2024::Day1::run(&input),
        _ => println!("Day not found"),
    }
}

struct Args {
    day: usize,
    input_path: String,
}

impl Args {
    fn from_env() -> Self {
        let mut raw_args = std::env::args();

        // Skip first cause its the path to the executable
        raw_args.next();

        let raw_day = raw_args.next().expect("No Days Argument e.g --day=1");
        let input_path = raw_args.next().expect("No Input Path");

        let day = raw_day
            .split("=")
            .last()
            .expect("Could not parse Day argument. E.g --day=1")
            .parse::<usize>()
            .expect("Could not parse the day number");

        Args { day, input_path }
    }
}
