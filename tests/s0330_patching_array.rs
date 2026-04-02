// Tests for Problem 0330: Patching Array
// Java reference: src/test/java/g0301_0400/s0330_patching_array/SolutionTest.java

use leetcode_in_rust::s0330::patching_array::Solution;

#[test]
fn test_min_patches() {
    assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
}

#[test]
fn test_min_patches2() {
    assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
}

#[test]
fn test_min_patches3() {
    assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
}
