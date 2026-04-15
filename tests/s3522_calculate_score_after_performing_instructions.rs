// Tests for Problem 3522: Calculate Score After Performing Instructions
// Java reference: src/test/java/g3501_3600/s3522_calculate_score_after_performing_instructions/SolutionTest.java

use leetcode_in_rust::s3522::calculate_score_after_performing_instructions::Solution;

#[test]
fn test_calculate_score() {
    assert_eq!(
        Solution::calculate_score(
            vec!["jump".to_string(), "add".to_string(), "add".to_string(), "jump".to_string(), "add".to_string(), "jump".to_string()],
            vec![2, 1, 3, 1, -2, -3]
        ),
        1i64
    );
}

#[test]
fn test_calculate_score2() {
    assert_eq!(
        Solution::calculate_score(vec!["jump".to_string(), "add".to_string(), "add".to_string()], vec![3, 1, 1]),
        0i64
    );
}

#[test]
fn test_calculate_score3() {
    assert_eq!(
        Solution::calculate_score(vec!["jump".to_string()], vec![0]),
        0i64
    );
}
