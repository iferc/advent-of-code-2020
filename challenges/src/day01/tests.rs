// import everything from the parent module file (in this case mod.rs)
use super::*;

const SAMPLE_DATA: &'static str = "1721
979
366
299
675
1456";

#[test]
fn sample_data_builds_ok() {
    // unwrap will fail the test here if the day input parsing returned an Err
    Day01::new(SAMPLE_DATA).unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result() {
    let mut challenge = Day01::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_silver().unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day01::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_silver();

    assert_eq!(
        result,
        Ok(SilverSolution {
            numbers: [299, 1721],
            result: 514579,
        })
    );
}

#[test]
fn sample_data_for_gold_solution_has_ok_result() {
    let mut challenge = Day01::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_gold().unwrap();
}

#[test]
fn sample_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day01::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_gold();

    assert_eq!(
        result,
        Ok(GoldSolution {
            numbers: [366, 675, 979],
            result: 241861950,
        })
    );
}
