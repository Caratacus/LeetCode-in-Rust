// Tests for Problem 1818: Minimum Absolute Sum Difference
// Java reference: src/test/java/g1801_1900/s1818_minimum_absolute_sum_difference/SolutionTest.java

use leetcode_in_rust::s1818::minimum_absolute_sum_difference::Solution;

#[test]
fn test_min_absolute_sum_diff() {
    assert_eq!(
        Solution::min_absolute_sum_diff(vec![1, 7, 5], vec![2, 3, 5]),
        3
    );
}

#[test]
fn test_min_absolute_sum_diff2() {
    assert_eq!(
        Solution::min_absolute_sum_diff(vec![2, 4, 6, 8, 10], vec![2, 4, 6, 8, 10]),
        0
    );
}

#[test]
fn test_min_absolute_sum_diff3() {
    assert_eq!(
        Solution::min_absolute_sum_diff(vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4]),
        20
    );
}
