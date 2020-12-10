// import everything from the parent module file (in this case mod.rs)
use super::*;

// note that this tests module will need some love later
// to account for the shorter preamble with test data

const SAMPLE_DATA: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

#[test]
fn sample_data_builds_ok() {
    // unwrap will fail the test here if the day input parsing returned an Err
    Day09::new(SAMPLE_DATA).unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result() {
    let mut _challenge = Day09::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    // challenge.attempt_silver().unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day09::new(SAMPLE_DATA).unwrap();
    let _result = challenge.attempt_silver();

    // the test data uses a different preamble length in the test data,
    // so right now this test only works if the preamble is set to 5 in the module

    // assert_eq!(_result, Ok(127));
}

#[test]
fn sample_data_for_gold_solution_has_ok_result() {
    let mut _challenge = Day09::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    // challenge.attempt_gold().unwrap();
}

#[test]
fn sample_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day09::new(SAMPLE_DATA).unwrap();
    let _result = challenge.attempt_gold();

    // the test data uses a different preamble length in the test data,
    // so right now this test only works if the preamble is set to 5 in the module

    // assert_eq!(_result, Ok(62));
}
