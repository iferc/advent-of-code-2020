// import everything from the parent module file (in this case mod.rs)
use super::{password::*, *};

#[test]
fn sample_1_parses_as_1_valid() {
    let input = "1-3 a: abcde";
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![Password {
                policy: PasswordPolicy {
                    amount_range: 1..3,
                    symbol: "a".to_string(),
                },
                password: "abcde".to_string(),
            }],
        })
    );
}

#[test]
fn sample_2_parses_as_0_valid() {
    let input = "1-3 b: cdefg";
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![Password {
                policy: PasswordPolicy {
                    amount_range: 1..3,
                    symbol: "b".to_string(),
                },
                password: "cdefg".to_string(),
            }],
        })
    );
}

#[test]
fn sample_3_parses_as_1_valid() {
    let input = "2-9 c: ccccccccc";
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![Password {
                policy: PasswordPolicy {
                    amount_range: 2..9,
                    symbol: "c".to_string(),
                },
                password: "ccccccccc".to_string(),
            }],
        })
    );
}

#[test]
fn sample_all_1_through_3_parses_as_2_valid() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![
                Password {
                    policy: PasswordPolicy {
                        amount_range: 1..3,
                        symbol: "a".to_string(),
                    },
                    password: "abcde".to_string(),
                },
                Password {
                    policy: PasswordPolicy {
                        amount_range: 1..3,
                        symbol: "b".to_string(),
                    },
                    password: "cdefg".to_string(),
                },
                Password {
                    policy: PasswordPolicy {
                        amount_range: 2..9,
                        symbol: "c".to_string(),
                    },
                    password: "ccccccccc".to_string(),
                }
            ],
        })
    );
}

#[test]
fn sample_1_silver_parses_as_1_valid() {
    let input = "1-3 a: abcde";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(1));
}

#[test]
fn sample_2_silver_parses_as_0_valid() {
    let input = "1-3 b: cdefg";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(0));
}

#[test]
fn sample_3_silver_parses_as_1_valid() {
    let input = "2-9 c: ccccccccc";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(1));
}

#[test]
fn sample_all_silver_1_through_3_parses_as_2_valid() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(2));
}

#[test]
fn sample_1_gold_parses_as_1_valid() {
    let input = "1-3 a: abcde";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(1));
}

#[test]
fn sample_2_gold_parses_as_0_valid() {
    let input = "1-3 b: cdefg";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(0));
}

#[test]
fn sample_3_gold_parses_as_1_valid() {
    let input = "2-9 c: ccccccccc";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(0));
}

#[test]
fn sample_all_gold_1_through_3_parses_as_2_valid() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(1));
}
