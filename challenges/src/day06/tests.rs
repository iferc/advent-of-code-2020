// import everything from the parent module file (in this case mod.rs)
use super::*;

const SAMPLE_DATA_1: &'static str = "abcx
abcy
abcz";

const SAMPLE_DATA_2: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

#[test]
fn sample_data_builds_ok() {
    // unwrap will fail the test here if the day input parsing returned an Err
    Day06::new(SAMPLE_DATA_1).unwrap();
    Day06::new(SAMPLE_DATA_2).unwrap();
}

#[test]
fn sample_1_data_for_silver_solution_has_ok_result() {
    let mut challenge = Day06::new(SAMPLE_DATA_1).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_silver().unwrap();
}

#[test]
fn sample_1_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day06::new(SAMPLE_DATA_1).unwrap();
    let result = challenge.attempt_silver();

    assert_eq!(result, Ok(6));
}

#[test]
fn sample_2_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day06::new(SAMPLE_DATA_2).unwrap();
    let result = challenge.attempt_silver();

    assert_eq!(result, Ok(11));
}

#[test]
fn sample_1_data_for_gold_solution_has_ok_result() {
    let mut challenge = Day06::new(SAMPLE_DATA_1).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_gold().unwrap();
}

#[test]
fn sample_1_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day06::new(SAMPLE_DATA_1).unwrap();
    let result = challenge.attempt_gold();

    assert_eq!(result, Ok(3));
}

#[test]
fn sample_2_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day06::new(SAMPLE_DATA_2).unwrap();
    let result = challenge.attempt_gold();

    assert_eq!(result, Ok(6));
}
