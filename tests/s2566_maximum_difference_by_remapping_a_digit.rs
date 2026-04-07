// Tests for Problem 2566: Maximum Difference by Remapping a Digit
// Java reference: src/test/java/g2501_2600/s2566_maximum_difference_by_remapping_a_digit/SolutionTest.java

use leetcode_in_rust::s2566::maximum_difference_by_remapping_a_digit::Solution;

#[test]
fn test_min_max_difference() {
    assert_eq!(Solution::min_max_difference(11891), 99009);
}

#[test]
fn test_min_max_difference2() {
    assert_eq!(Solution::min_max_difference(90), 99);
}
