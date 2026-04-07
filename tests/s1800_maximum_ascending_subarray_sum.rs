// Tests for Problem 1800: Maximum Ascending Subarray Sum
// Java reference: src/test/java/g1701_1800/s1800_maximum_ascending_subarray_sum/SolutionTest.java

use leetcode_in_rust::s1800::maximum_ascending_subarray_sum::Solution;

#[test]
fn test_max_ascending_sum() {
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
}

#[test]
fn test_max_ascending_sum2() {
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
}

#[test]
fn test_max_ascending_sum3() {
    assert_eq!(
        Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
        33
    );
}
