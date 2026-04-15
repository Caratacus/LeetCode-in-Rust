// Tests for Problem 3411: Maximum Subarray with Equal Products
// Java reference: src/test/java/g3401_3500/s3411_maximum_subarray_with_equal_products/SolutionTest.java

use leetcode_in_rust::s3411::maximum_subarray_with_equal_products::Solution;

#[test]
fn test_max_length() {
    assert_eq!(Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]), 5);
}

#[test]
fn test_max_length2() {
    assert_eq!(Solution::max_length(vec![2, 3, 4, 5, 6]), 3);
}

#[test]
fn test_max_length3() {
    assert_eq!(Solution::max_length(vec![1, 2, 3, 1, 4, 5, 1]), 5);
}
