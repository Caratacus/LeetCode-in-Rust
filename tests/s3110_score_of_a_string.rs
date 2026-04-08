// Tests for Problem 3110: Score of a String
// Java reference: src/test/java/g3101_3200/s3110_score_of_a_string/SolutionTest.java

use leetcode_in_rust::s3110::score_of_a_string::Solution;

#[test]
fn test_score_of_string() {
    assert_eq!(Solution::score_of_string("hello".to_string()), 13);
}

