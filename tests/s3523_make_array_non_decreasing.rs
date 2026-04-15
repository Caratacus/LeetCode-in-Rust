// Tests for Problem 3523: Make Array Non Decreasing
// Java reference: src/test/java/g3501_3600/s3523_make_array_non_decreasing/SolutionTest.java

use leetcode_in_rust::s3523::make_array_non_decreasing::Solution;

#[test]
fn test_maximum_possible_size() {
    assert_eq!(Solution::maximum_possible_size(vec![4, 2, 5, 3, 5]), 3);
}

#[test]
fn test_maximum_possible_size2() {
    assert_eq!(Solution::maximum_possible_size(vec![1, 2, 3]), 3);
}
