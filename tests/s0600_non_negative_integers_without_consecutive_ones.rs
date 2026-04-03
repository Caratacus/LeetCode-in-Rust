// Tests for Problem 0600: Non-negative Integers without Consecutive Ones
// Java reference: src/test/java/g0501_0600/s0600_non_negative_integers_without_consecutive_ones/SolutionTest.java

use leetcode_in_rust::s0600::non_negative_integers_without_consecutive_ones::Solution;

#[test]
fn test_find_integers() {
    assert_eq!(Solution::find_integers(5), 5);
}

#[test]
fn test_find_integers2() {
    assert_eq!(Solution::find_integers(100000000), 514229);
}
