// import everything from the parent module file (in this case mod.rs)
use super::*;

const SAMPLE_DATA: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[test]
fn sample_data_builds_ok() {
    // unwrap will fail the test here if the day input parsing returned an Err
    Day07::new(SAMPLE_DATA).unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result() {
    let mut challenge = Day07::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_silver().unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day07::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_silver();

    assert_eq!(result, Ok(4));
}

#[test]
fn sample_data_for_gold_solution_has_ok_result() {
    let mut challenge = Day07::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_gold().unwrap();
}

#[test]
fn sample_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day07::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_gold();

    assert_eq!(result, Ok(32));
}
