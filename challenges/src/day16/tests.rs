// import everything from the parent module file (in this case mod.rs)
use super::*;

const SAMPLE_DATA: &'static str = "<replace this with sample data from challenge>";

#[test]
fn sample_data_builds_ok() {
    // unwrap will fail the test here if the day input parsing returned an Err
    Day16::new(SAMPLE_DATA).unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result() {
    let mut challenge = Day16::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_silver().unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day16::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_silver();

    // replace the unit value `()` with the correct result based on the sample data
    assert_eq!(result, Ok(()));
}

#[test]
fn sample_data_for_gold_solution_has_ok_result() {
    let mut challenge = Day16::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_gold().unwrap();
}

#[test]
fn sample_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day16::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_gold();

    // replace the unit value `()` with the correct result based on the sample data
    assert_eq!(result, Ok(()));
}
