// Tests for Problem 3176: Find the Maximum Length of a Good Subsequence I
// Java reference: src/test/java/g3101_3200/s3176_find_the_maximum_length_of_a_good_subsequence_i/SolutionTest.java

use leetcode_in_rust::s3176::find_the_maximum_length_of_a_good_subsequence_i::Solution;
use std::collections::HashMap;

#[test]
fn test_maximum_length() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
}
#[test]
fn test_maximum_length2() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
}