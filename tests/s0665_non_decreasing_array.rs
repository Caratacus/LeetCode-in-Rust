// Tests for Problem 0665: Non-decreasing Array
// Java reference: src/test/java/g0601_0700/s0665_non_decreasing_array/SolutionTest.java

use leetcode_in_rust::s0665::non_decreasing_array::Solution;

#[test]
fn test_check_possibility() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
}

#[test]
fn test_check_possibility2() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
}
