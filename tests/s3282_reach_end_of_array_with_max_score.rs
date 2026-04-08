// Tests for Problem 3282: Reach End of Array With Max Score
// Java reference: src/test/java/g3201_3300/s3282_reach_end_of_array_with_max_score/SolutionTest.java

use leetcode_in_rust::s3282::reach_end_of_array_with_max_score::Solution;

#[test]
fn test_find_maximum_score() {
    assert_eq!(Solution::find_maximum_score(vec![1, 3, 1, 5]), 7);
}

#[test]
fn test_find_maximum_score2() {
    assert_eq!(Solution::find_maximum_score(vec![4, 3, 1, 3, 2]), 16);
}
