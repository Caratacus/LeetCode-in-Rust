// Tests for Problem 3434: Maximum Frequency After Subarray Operation
// Java reference: src/test/java/g3401_3500/s3434_maximum_frequency_after_subarray_operation/SolutionTest.java

use leetcode_in_rust::s3434::maximum_frequency_after_subarray_operation::Solution;

#[test]
fn test_max_frequency() {
    assert_eq!(Solution::max_frequency(vec![1, 2, 3, 4, 5, 6], 1), 2);
}

#[test]
fn test_max_frequency2() {
    assert_eq!(Solution::max_frequency(vec![10, 2, 3, 4, 5, 5, 4, 3, 2, 2], 10), 4);
}
