// import everything from the parent module file (in this case mod.rs)
use super::*;

const SAMPLE_DATA: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

#[test]
fn sample_data_builds_ok() {
    // unwrap will fail the test here if the day input parsing returned an Err
    Day03::new(SAMPLE_DATA).unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_silver().unwrap();
}

#[test]
fn sample_data_for_silver_solution_has_ok_result_of() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_silver();

    assert_eq!(result, Ok(7));
}

#[test]
fn sample_data_for_gold_solution_has_ok_result() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();

    // unwrap will fail the test here if the silver challenge returned an Err
    challenge.attempt_gold().unwrap();
}

#[test]
fn sample_data_for_gold_solution_has_ok_result_of() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();
    let result = challenge.attempt_gold();

    assert_eq!(result, Ok(336));
}

#[test]
fn sample_map_navigation() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();

    assert_eq!(challenge.tile(), Some(&Tile::Open));
    assert_eq!(challenge.navigate(3, 1).tile(), Some(&Tile::Open));
    assert_eq!(challenge.navigate(1, 0).tile(), Some(&Tile::Tree));
    assert_eq!(challenge.navigate(2, 1).tile(), Some(&Tile::Tree));
    assert_eq!(challenge.location(), &MapLocation { right: 6, down: 2 });
}

#[test]
fn sample_map_navigation_past_edge_of_map() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();
    assert_eq!(challenge.navigate(13, 0).tile(), Some(&Tile::Tree));

    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();
    assert_eq!(challenge.navigate(14, 0).tile(), Some(&Tile::Tree));

    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();
    assert_eq!(challenge.navigate(15, 0).tile(), Some(&Tile::Open));
}

#[test]
fn sample_map_navigation_past_bottom_of_map() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();
    assert_eq!(challenge.navigate(0, 11).tile(), None);
}

#[test]
fn sample_map_location() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();

    assert_eq!(
        challenge
            .navigate(3, 1)
            .navigate(1, 0)
            .navigate(2, 1)
            .location(),
        &MapLocation { right: 6, down: 2 }
    );
}

#[test]
fn sample_map_gold_solution() {
    let mut challenge = Day03::new(SAMPLE_DATA).unwrap();

    assert_eq!(challenge.attempt_gold(), Ok(336));
}
