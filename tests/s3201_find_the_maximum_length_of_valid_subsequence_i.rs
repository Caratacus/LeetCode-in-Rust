// Tests for Problem 3201: Find the Maximum Length of Valid Subsequence I
// Java reference: src/test/java/g3201_3300/s3201_find_the_maximum_length_of_valid_subsequence_i/SolutionTest.java

use leetcode_in_rust::s3201::find_the_maximum_length_of_valid_subsequence_i::Solution;

#[test]
fn test_maximum_length() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4]), 4);
}

#[test]
fn test_maximum_length2() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]), 6);
}
