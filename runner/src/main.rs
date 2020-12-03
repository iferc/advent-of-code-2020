use advent_of_code_2020_challenges::*;
use separator::Separatable;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Advent of Code 2020")]
struct Opt {
    /// Which day of the challenge to run, runs all by default
    #[structopt(long, env = "CHALLENGE_DAY")]
    day: Option<u32>,

    /// Input data file, defaults to input stored with date of challenge
    #[structopt(short, long, env = "INPUT_FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Input string, defaults to input stored with date of challenge
    #[structopt(short, long, env = "INPUT_DATA")]
    data: Option<String>,
}

fn read_file(file_path: PathBuf) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn solve_challenge<D>(
    possible_challenge: Result<D, String>,
    data_parse_start_time: Instant,
) -> Result<(), String>
where
    D: SilverChallenge + GoldChallenge + std::fmt::Debug,
    <D as SilverChallenge>::Answer: std::fmt::Debug,
    <D as GoldChallenge>::Answer: std::fmt::Debug,
{
    let challenge = match possible_challenge {
        Err(error) => return Err(error),
        Ok(challenge) => challenge,
    };
    let data_parse_total_time = data_parse_start_time.elapsed();
    println!(
        "-> Input data <-\nProcessing time: {} μs",
        data_parse_total_time.as_micros().separated_string()
    );

    let silver_start_time = Instant::now();
    let silver_solution = challenge.attempt_silver();
    let silver_total_time = silver_start_time.elapsed();
    println!(
        "-> Silver <-\nProcessing time: {} μs\n{:#?}",
        silver_total_time.as_micros().separated_string(),
        silver_solution
    );

    let gold_start_time = Instant::now();
    let gold_solution = challenge.attempt_gold();
    let gold_total_time = gold_start_time.elapsed();
    println!(
        "-> Gold <-\nProcessing time: {} μs\n{:#?}",
        gold_total_time.as_micros().separated_string(),
        gold_solution
    );

    Ok(())
}

fn attempt_challenge_on_date(day: u32, data_override: &Option<String>) -> Result<(), String> {
    let data = match data_override {
        Some(data) => data.clone(),
        None => {
            let mut file_path = PathBuf::new();
            file_path.push("input");
            file_path.push(format!("day{:02}.txt", day));

            match read_file(file_path) {
                Err(_) => return Err(format!("Unable to read input file for day {}.", day)),
                Ok(file_data) => file_data,
            }
        }
    };

    println!("===> Day {} <===", day);
    let data_parse_start_time = Instant::now();
    match day {
        1 => solve_challenge(Day01::new(data), data_parse_start_time)?,
        2 => solve_challenge(Day02::new(data), data_parse_start_time)?,
        3 => solve_challenge(Day03::new(data), data_parse_start_time)?,
        4 => solve_challenge(Day04::new(data), data_parse_start_time)?,
        5 => solve_challenge(Day05::new(data), data_parse_start_time)?,
        6 => solve_challenge(Day06::new(data), data_parse_start_time)?,
        7 => solve_challenge(Day07::new(data), data_parse_start_time)?,
        8 => solve_challenge(Day08::new(data), data_parse_start_time)?,
        9 => solve_challenge(Day09::new(data), data_parse_start_time)?,
        10 => solve_challenge(Day10::new(data), data_parse_start_time)?,
        11 => solve_challenge(Day11::new(data), data_parse_start_time)?,
        12 => solve_challenge(Day12::new(data), data_parse_start_time)?,
        13 => solve_challenge(Day13::new(data), data_parse_start_time)?,
        14 => solve_challenge(Day14::new(data), data_parse_start_time)?,
        15 => solve_challenge(Day15::new(data), data_parse_start_time)?,
        16 => solve_challenge(Day16::new(data), data_parse_start_time)?,
        17 => solve_challenge(Day17::new(data), data_parse_start_time)?,
        18 => solve_challenge(Day18::new(data), data_parse_start_time)?,
        19 => solve_challenge(Day19::new(data), data_parse_start_time)?,
        20 => solve_challenge(Day20::new(data), data_parse_start_time)?,
        21 => solve_challenge(Day21::new(data), data_parse_start_time)?,
        22 => solve_challenge(Day22::new(data), data_parse_start_time)?,
        23 => solve_challenge(Day23::new(data), data_parse_start_time)?,
        24 => solve_challenge(Day24::new(data), data_parse_start_time)?,
        25 => solve_challenge(Day25::new(data), data_parse_start_time)?,
        _ => return Err(format!("Unrecognized date given: {}.", day)),
    };

    Ok(())
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    let data_override: Option<String> = match (opt.day, opt.data, opt.file) {
        (None, Some(_), _) | (None, _, Some(_)) => {
            return Err("Must specify date when providing data or file.".into())
        }

        (_, Some(_), Some(_)) => return Err("Cannot specify both data and file.".into()),

        (Some(_), None, Some(file_path)) => match read_file(file_path) {
            Err(_) => return Err("Unable to read input file.".into()),
            Ok(file_data) => Some(file_data),
        },

        (Some(_), Some(data), None) => Some(data),

        // no overrides specified for data or file
        (_, None, None) => None,
    };

    if let Some(day) = opt.day {
        return attempt_challenge_on_date(day, &data_override);
    }

    for day in 1..=25 {
        attempt_challenge_on_date(day, &data_override).ok();
    }

    Ok(())
}
