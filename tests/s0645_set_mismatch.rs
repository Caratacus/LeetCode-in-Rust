// Tests for Problem 0645: Set Mismatch
// Java reference: src/test/java/g0601_0700/s0645_set_mismatch/SolutionTest.java

use leetcode_in_rust::s0645::set_mismatch::Solution;

#[test]
fn test_find_error_nums() {
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
}

#[test]
fn test_find_error_nums2() {
    assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
}
