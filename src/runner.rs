use clap::{crate_name, Arg, Command};
use log::Level;
use std::time::Instant;

#[allow(clippy::missing_panics_doc)]
pub fn run(year: &'static str, solutions: [&dyn Solution; 25]) {
    let matches = Command::new(crate_name!())
        .about(format!("Advent of Code {year}"))
        .arg(
            Arg::new("day")
                .value_parser(clap::value_parser!(u32).range(1..=25))
                .required(true),
        )
        .arg(Arg::new("puzzle").value_parser(["1", "2"]))
        .arg(
            Arg::new("log_level")
                .long("level")
                .help("Logging level")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .default_value("warn"),
        )
        .get_matches();

    let level_match = &matches
        .get_one("log_level")
        .map(|it: &String| it.to_lowercase())
        .unwrap();
    let log_level = match level_match.as_str() {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => unreachable!(),
    };

    simple_logger::init_with_level(log_level).unwrap();

    let day: usize = matches.get_one("day").map(|it: &u32| *it as usize).unwrap();
    let puzzle = matches.get_one("puzzle").map(String::as_str);

    let solution = solutions[day - 1];

    let (answer, time) = match puzzle {
        Some("1") => {
            let input = read_input(day, "1");
            time(|| solution.solve_1(input))
        }
        Some("2") => {
            let input = read_input(day, "2");
            time(|| solution.solve_2(input))
        }
        Some(_) => unreachable!(),
        None => {
            let input_1 = read_input(day, "1");
            let input_2 = read_input(day, "2");
            let ((a1, a2), time) = time(|| (solution.solve_1(input_1), solution.solve_2(input_2)));
            (a1 + "\n" + &a2, time)
        }
    };

    println!("{answer}");
    println!("Elapsed: {time}ms");
}

/**
# Panics
If the input file is not found within the input folder
 */
pub fn read_input(day: usize, puzzle: &str) -> String {
    let dss = format!("input/{day}-{puzzle}.txt");
    let ds = std::path::Path::new(&dss);
    if ds.exists() {
        std::fs::read_to_string(ds).unwrap()
    } else {
        std::fs::read_to_string(format!("input/{day}.txt")).unwrap()
    }
}

fn time<T>(f: impl FnOnce() -> T) -> (T, u128) {
    let start = Instant::now();
    let res = f();
    (res, start.elapsed().as_millis())
}

pub trait Solution {
    fn solve_1(&self, input: String) -> String;
    fn solve_2(&self, input: String) -> String;
}

#[allow(clippy::missing_panics_doc)]
pub fn init_test_logging() {
    use std::sync::Once;
    static TEST_INIT: Once = Once::new();
    TEST_INIT.call_once(|| simple_logger::init_with_level(Level::Trace).unwrap());
}
