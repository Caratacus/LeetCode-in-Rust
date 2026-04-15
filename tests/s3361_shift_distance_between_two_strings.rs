// Tests for Problem 3361: Shift Distance Between Two Strings
// Java reference: src/test/java/g3301_3400/s3361_shift_distance_between_two_strings/SolutionTest.java

use leetcode_in_rust::s3361::shift_distance_between_two_strings::Solution;

#[test]
fn test_shift_distance() {
    let next_cost = vec![
        100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let previous_cost = vec![
        1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(
        Solution::shift_distance(
            "abab".to_string(),
            "baba".to_string(),
            next_cost,
            previous_cost
        ),
        2
    );
}

#[test]
fn test_shift_distance2() {
    let next_cost = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let previous_cost = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    assert_eq!(
        Solution::shift_distance(
            "leet".to_string(),
            "code".to_string(),
            next_cost,
            previous_cost
        ),
        31
    );
}
