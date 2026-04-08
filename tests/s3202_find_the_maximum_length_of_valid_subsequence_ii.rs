// Tests for Problem 3202: Find the Maximum Length of Valid Subsequence II
// Java reference: src/test/java/g3201_3300/s3202_find_the_maximum_length_of_valid_subsequence_ii/SolutionTest.java

use leetcode_in_rust::s3202::find_the_maximum_length_of_valid_subsequence_ii::Solution;

#[test]
fn test_maximum_length() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
}

#[test]
fn test_maximum_length2() {
    assert_eq!(Solution::maximum_length(vec![1, 4, 2, 3, 1, 4], 3), 4);
}
