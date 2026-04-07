// Tests for Problem 1755: Closest Subsequence Sum
// Java reference: src/test/java/g1701_1800/s1755_closest_subsequence_sum/SolutionTest.java

use leetcode_in_rust::s1755::closest_subsequence_sum::Solution;

#[test]
fn test_min_abs_difference() {
    assert_eq!(Solution::min_abs_difference(vec![5, -7, 3, 5], 6), 0);
}

#[test]
fn test_min_abs_difference2() {
    assert_eq!(Solution::min_abs_difference(vec![7, -9, 15, -2], -5), 1);
}

#[test]
fn test_min_abs_difference3() {
    assert_eq!(Solution::min_abs_difference(vec![1, 2, 3], -7), 7);
}
