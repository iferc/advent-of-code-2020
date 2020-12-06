use advent_of_code_2020_challenges::*;
use separator::Separatable;
use std::time::Instant;

pub struct ChallengeOptions {
    show_timing: bool,
    show_solutions: bool,
    solve_silver: bool,
    solve_gold: bool,
}

impl ChallengeOptions {
    pub fn new(
        show_timing: bool,
        show_solutions: bool,
        solve_silver: bool,
        solve_gold: bool,
    ) -> Self {
        ChallengeOptions {
            show_timing,
            show_solutions,
            solve_silver,
            solve_gold,
        }
    }
}

pub fn attempt_challenges<D, F>(
    possible_challenge: F,
    options: &ChallengeOptions,
) -> Result<(), String>
where
    D: SilverChallenge + GoldChallenge + std::fmt::Debug,
    <D as SilverChallenge>::Answer: std::fmt::Debug,
    <D as SilverChallenge>::Error: std::fmt::Debug,
    <D as GoldChallenge>::Answer: std::fmt::Debug,
    <D as GoldChallenge>::Error: std::fmt::Debug,
    F: Fn() -> Result<D, String> + Sized,
{
    println!(" -> Input data");

    let data_parse_start_time = Instant::now();
    let mut challenge = match possible_challenge() {
        Err(error) => return Err(error),
        Ok(challenge) => challenge,
    };
    let data_parse_total_time = data_parse_start_time.elapsed();

    if options.show_timing {
        println!(
            "    Processing time: {:>10} μs",
            data_parse_total_time.as_micros().separated_string()
        );
    }

    if options.solve_silver {
        println!(" -> Silver");

        let silver_start_time = Instant::now();
        let silver_solution = challenge.attempt_silver();
        let silver_total_time = silver_start_time.elapsed();
        if options.show_timing {
            println!(
                "    Processing time: {:>10} μs",
                silver_total_time.as_micros().separated_string(),
            );
        }
        if options.show_solutions {
            println!("    Result: {:?}", silver_solution);
        }
    }

    if options.solve_gold {
        println!(" -> Gold");

        let gold_start_time = Instant::now();
        let gold_solution = challenge.attempt_gold();
        let gold_total_time = gold_start_time.elapsed();
        if options.show_timing {
            println!(
                "    Processing time: {:>10} μs",
                gold_total_time.as_micros().separated_string(),
            );
        }
        if options.show_solutions {
            println!("    Result: {:?}", gold_solution);
        }
    }

    Ok(())
}

pub fn attempt_challenges_for_day(
    day: &u32,
    options: &ChallengeOptions,
    data: String,
) -> Result<(), String> {
    println!("==> Day {}", day);

    let data_str = data.as_str();
    match day {
        // Day constructors are in a closure to defer instantiation for timing input parsing
        1 => attempt_challenges(|| Day01::new(data_str), options)?,
        2 => attempt_challenges(|| Day02::new(data_str), options)?,
        3 => attempt_challenges(|| Day03::new(data_str), options)?,
        4 => attempt_challenges(|| Day04::new(data_str), options)?,
        5 => attempt_challenges(|| Day05::new(data_str), options)?,
        6 => attempt_challenges(|| Day06::new(data_str), options)?,
        7 => attempt_challenges(|| Day07::new(data_str), options)?,
        8 => attempt_challenges(|| Day08::new(data_str), options)?,
        9 => attempt_challenges(|| Day09::new(data_str), options)?,
        10 => attempt_challenges(|| Day10::new(data_str), options)?,
        11 => attempt_challenges(|| Day11::new(data_str), options)?,
        12 => attempt_challenges(|| Day12::new(data_str), options)?,
        13 => attempt_challenges(|| Day13::new(data_str), options)?,
        14 => attempt_challenges(|| Day14::new(data_str), options)?,
        15 => attempt_challenges(|| Day15::new(data_str), options)?,
        16 => attempt_challenges(|| Day16::new(data_str), options)?,
        17 => attempt_challenges(|| Day17::new(data_str), options)?,
        18 => attempt_challenges(|| Day18::new(data_str), options)?,
        19 => attempt_challenges(|| Day19::new(data_str), options)?,
        20 => attempt_challenges(|| Day20::new(data_str), options)?,
        21 => attempt_challenges(|| Day21::new(data_str), options)?,
        22 => attempt_challenges(|| Day22::new(data_str), options)?,
        23 => attempt_challenges(|| Day23::new(data_str), options)?,
        24 => attempt_challenges(|| Day24::new(data_str), options)?,
        25 => attempt_challenges(|| Day25::new(data_str), options)?,
        _ => return Err(format!("Unrecognized date given: {}.", day)),
    };

    println!();
    Ok(())
}
