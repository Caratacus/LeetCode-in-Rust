// Tests for Problem 0075: Sort Colors
// Java reference: src/test/java/g0001_0100/s0075_sort_colors/SolutionTest.java

use leetcode_in_rust::s0075::sort_colors::Solution;

#[test]
fn test_sort_colors() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
}

#[test]
fn test_sort_colors2() {
    let mut nums = vec![2, 0, 1];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 1, 2]);
}
