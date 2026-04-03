// Tests for Problem 1752: Check if Array Is Sorted and Rotated
// Java reference: src/test/java/g1701_1800/s1752_check_if_array_is_sorted_and_rotated/SolutionTest.java

use leetcode_in_rust::s1752::check_if_array_is_sorted_and_rotated::Solution;

#[test]
fn test_check() {
    assert_eq!(Solution::check(vec![3, 4, 5, 1, 2]), true);
}

#[test]
fn test_check2() {
    assert_eq!(Solution::check(vec![2, 1, 3, 4]), false);
}

#[test]
fn test_check3() {
    assert_eq!(Solution::check(vec![1, 2, 3]), true);
}
