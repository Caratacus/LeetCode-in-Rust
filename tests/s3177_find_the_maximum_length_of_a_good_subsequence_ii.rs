// Tests for Problem 3177: Find the Maximum Length of a Good Subsequence II
// Java reference: src/test/java/g3101_3200/s3177_find_the_maximum_length_of_a_good_subsequence_ii/SolutionTest.java

use leetcode_in_rust::s3177::find_the_maximum_length_of_a_good_subsequence_ii::Solution;
use std::collections::HashMap;

#[test]
fn test_maximum_length() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
}
#[test]
fn test_maximum_length2() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
}