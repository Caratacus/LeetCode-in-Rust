// Tests for Problem 1590: Make Sum Divisible by P
// Java reference: src/test/java/g1501_1600/s1590_make_sum_divisible_by_p/SolutionTest.java

use leetcode_in_rust::s1590::make_sum_divisible_by_p::Solution;

#[test]
fn test_min_subarray() {
    assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
}

#[test]
fn test_min_subarray2() {
    assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
}

#[test]
fn test_min_subarray3() {
    assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
}
