// Tests for Problem 1438: Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
// Java reference: src/test/java/g1401_1500/s1438_longest_continuous_subarray_with_absolute_diff_less_than_or_equal_to_limit/SolutionTest.java

use leetcode_in_rust::s1438::longest_continuous_subarray_with_absolute_diff_less_than_or_equal_to_limit::Solution;

#[test]
fn test_longest_subarray() {
    assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
}

#[test]
fn test_longest_subarray2() {
    assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
}

#[test]
fn test_longest_subarray3() {
    assert_eq!(Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
}
