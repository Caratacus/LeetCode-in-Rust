// Tests for Problem 1422: Maximum Score After Splitting a String
// Java reference: src/test/java/g1401_1500/s1422_maximum_score_after_splitting_a_string/SolutionTest.java

use leetcode_in_rust::s1422::maximum_score_after_splitting_a_string::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score("011101".to_string()), 5);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score("00111".to_string()), 5);
}

#[test]
fn test_max_score3() {
    assert_eq!(Solution::max_score("1111".to_string()), 3);
}
