// Tests for Problem 0659: Split Array into Consecutive Subsequences
// Java reference: src/test/java/g0601_0700/s0659_split_array_into_consecutive_subsequences/SolutionTest.java

use leetcode_in_rust::s0659::split_array_into_consecutive_subsequences::Solution;

#[test]
fn test_is_possible() {
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]), true);
}

#[test]
fn test_is_possible2() {
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 4, 4, 5]), false);
}

#[test]
fn test_is_possible3() {
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
}
