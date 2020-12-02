use advent_of_code_2020_challenges::*;
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

fn solve_challenge<D>(possible_challenge: Result<D, String>) -> Result<(), String>
where
    D: SilverChallenge + GoldChallenge + std::fmt::Debug,
    <D as SilverChallenge>::Answer: std::fmt::Debug,
    <D as GoldChallenge>::Answer: std::fmt::Debug,
{
    let data_parse_start_time = Instant::now();
    let challenge = match possible_challenge {
        Err(error) => return Err(error),
        Ok(challenge) => challenge,
    };
    let data_parse_total_time = data_parse_start_time.elapsed();
    println!(
        "-> Input data <-\nProcessing time: {} ns",
        data_parse_total_time.as_nanos()
    );

    let silver_start_time = Instant::now();
    let silver_solution = challenge.attempt_silver();
    let silver_total_time = silver_start_time.elapsed();
    println!(
        "-> Silver <-\nProcessing time: {} ns\n{:#?}",
        silver_total_time.as_nanos(),
        silver_solution
    );

    let gold_start_time = Instant::now();
    let gold_solution = challenge.attempt_gold();
    let gold_total_time = gold_start_time.elapsed();
    println!(
        "-> Gold <-\nProcessing time: {} ns\n{:#?}",
        gold_total_time.as_nanos(),
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
    match day {
        1 => solve_challenge(Day01::new(data))?,
        2 => solve_challenge(Day02::new(data))?,
        3 => solve_challenge(Day03::new(data))?,
        4 => solve_challenge(Day04::new(data))?,
        5 => solve_challenge(Day05::new(data))?,
        6 => solve_challenge(Day06::new(data))?,
        7 => solve_challenge(Day07::new(data))?,
        8 => solve_challenge(Day08::new(data))?,
        9 => solve_challenge(Day09::new(data))?,
        10 => solve_challenge(Day10::new(data))?,
        11 => solve_challenge(Day11::new(data))?,
        12 => solve_challenge(Day12::new(data))?,
        13 => solve_challenge(Day13::new(data))?,
        14 => solve_challenge(Day14::new(data))?,
        15 => solve_challenge(Day15::new(data))?,
        16 => solve_challenge(Day16::new(data))?,
        17 => solve_challenge(Day17::new(data))?,
        18 => solve_challenge(Day18::new(data))?,
        19 => solve_challenge(Day19::new(data))?,
        20 => solve_challenge(Day20::new(data))?,
        21 => solve_challenge(Day21::new(data))?,
        22 => solve_challenge(Day22::new(data))?,
        23 => solve_challenge(Day23::new(data))?,
        24 => solve_challenge(Day24::new(data))?,
        25 => solve_challenge(Day25::new(data))?,
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
