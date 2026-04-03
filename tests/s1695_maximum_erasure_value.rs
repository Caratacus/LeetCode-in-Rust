// Tests for Problem 1695: Maximum Erasure Value
// Java reference: src/test/java/g1601_1700/s1695_maximum_erasure_value/SolutionTest.java

use leetcode_in_rust::s1695::maximum_erasure_value::Solution;

#[test]
fn test_maximum_unique_subarray() {
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
}

#[test]
fn test_maximum_unique_subarray2() {
    assert_eq!(Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
}
