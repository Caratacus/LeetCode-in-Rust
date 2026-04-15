// Tests for Problem 3412: Find Mirror Score of a String
// Java reference: src/test/java/g3401_3500/s3412_find_mirror_score_of_a_string/SolutionTest.java

use leetcode_in_rust::s3412::find_mirror_score_of_a_string::Solution;

#[test]
fn test_calculate_score() {
    assert_eq!(Solution::calculate_score("aczzx".to_string()), 5i64);
}

#[test]
fn test_calculate_score2() {
    assert_eq!(Solution::calculate_score("abcdef".to_string()), 0i64);
}
