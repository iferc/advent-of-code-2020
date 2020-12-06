mod challenges;
use challenges::{attempt_challenges_for_day, ChallengeOptions};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Advent of Code 2020")]
struct ApplicationOptions {
    /// Which day of the challenge to run, runs all by default
    #[structopt(long, env = "CHALLENGE_DAY")]
    day: Option<u32>,

    /// Input data file, defaults to input stored with date of challenge
    #[structopt(short, long, env = "INPUT_FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Input string, defaults to input stored with date of challenge
    #[structopt(short, long, env = "INPUT_DATA")]
    data: Option<String>,

    /// Hide solutions
    #[structopt(long)]
    hide_solutions: bool,

    /// Hide time tracking
    #[structopt(long)]
    hide_timing: bool,

    /// Solve silver challenges, will solve both by default
    #[structopt(short = "s", long)]
    solve_silver: bool,

    /// Solve gold challenges, will solve both by default
    #[structopt(short = "g", long)]
    solve_gold: bool,
}

fn build_challenge_options_for_day(options: &ApplicationOptions) -> ChallengeOptions {
    let (solve_silver, solve_gold) = match (options.solve_silver, options.solve_gold) {
        // if none specified, run both
        (false, false) => (true, true),
        // otherwise solve whichever are specified
        (silver, gold) => (silver, gold),
    };
    ChallengeOptions::new(
        !options.hide_timing,
        !options.hide_solutions,
        solve_silver,
        solve_gold,
    )
}

fn read_file(file_path: PathBuf) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

// gather data for test between overridden data or input text file
fn prepare_challenge_data_for_day(
    day: &u32,
    possible_data_override: &Option<String>,
) -> Result<String, String> {
    match possible_data_override {
        Some(data) => Ok(data.clone()),
        None => {
            let mut file_path = PathBuf::new();
            file_path.push("input");
            file_path.push(format!("day{:02}.txt", day));

            match read_file(file_path) {
                Err(_) => Err(format!("Unable to read input file for day {}.", day)),
                Ok(file_data) => Ok(file_data),
            }
        }
    }
}

fn main() -> Result<(), String> {
    let options = ApplicationOptions::from_args();
    let challenge_options = build_challenge_options_for_day(&options);

    // conditions for whether to use input data or a file, or default file(s)
    let possible_data_override: Option<String> = match (options.day, &options.data, &options.file) {
        (None, Some(_), _) | (None, _, Some(_)) => {
            return Err("Must specify date when providing data or file.".into())
        }

        (_, Some(_), Some(_)) => return Err("Cannot specify both data and file.".into()),

        (Some(_), None, Some(file_path)) => match read_file(file_path.clone()) {
            Err(_) => return Err("Unable to read input file.".into()),
            Ok(file_data) => Some(file_data),
        },

        (Some(_), Some(data), None) => Some(data.clone()),

        // no overrides specified for data or file
        (_, None, None) => None,
    };

    // when a specific day is specified, only that days challenges will run
    // and if there is no input for that day, the application will terminate
    if let Some(day) = &options.day {
        let data = prepare_challenge_data_for_day(day, &possible_data_override).unwrap();
        return attempt_challenges_for_day(day, &challenge_options, data);
    }

    // when running through all days, will ignore days where no input exists
    // and will not end the application when a challenge fails
    for day in 1..=25 {
        if let Ok(data) = prepare_challenge_data_for_day(&day, &possible_data_override) {
            if let Err(error) = attempt_challenges_for_day(&day, &challenge_options, data) {
                eprintln!("Failed to attempt challenge for day {}: {}", day, error);
            }
        }
    }

    Ok(())
}
