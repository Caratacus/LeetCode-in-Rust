// Tests for Problem 2841: Maximum Sum of Almost Unique Subarray
// Java reference: src/test/java/g2801_2900/s2841_maximum_sum_of_almost_unique_subarray/SolutionTest.java

use leetcode_in_rust::s2841::maximum_sum_of_almost_unique_subarray::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4), 18);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![5, 9, 9, 2, 4, 5, 4], 1, 3), 23);
}

#[test]
fn test_max_sum3() {
    assert_eq!(Solution::max_sum(vec![1, 2, 1, 2, 1, 2, 1], 3, 3), 0);
}
