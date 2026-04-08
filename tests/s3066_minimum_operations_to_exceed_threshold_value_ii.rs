// Tests for Problem 3066: Minimum Operations to Exceed Threshold Value II
// Java reference: src/test/java/g3001_3100/s3066_minimum_operations_to_exceed_threshold_value_ii/SolutionTest.java

use leetcode_in_rust::s3066::minimum_operations_to_exceed_threshold_value_ii::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 2);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 20), 4);
}
