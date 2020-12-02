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

fn attempt_challenge(day: u32, data_override: &Option<String>) -> Result<(), String> {
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

    let data_parse_start_time = Instant::now();
    let possible_challenge = match day {
        1 => Day01::new(data),
        _ => return Err(format!("Unrecognized date given: {}.", day)),
    };
    let challenge = match possible_challenge {
        Err(error) => return Err(error),
        Ok(challenge) => challenge,
    };
    let data_parse_total_time = data_parse_start_time.elapsed();

    println!(
        "===> Day {} <===\n-> Input data <-\nProcessing time: {} ns",
        day,
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
        return attempt_challenge(day, &data_override);
    }

    for day in 1..=1 {
        attempt_challenge(day, &data_override)?;
    }

    Ok(())
}
