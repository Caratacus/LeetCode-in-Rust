// Tests for Problem 2235: Add Two Integers
// Java reference: src/test/java/g2201_2300/s2235_add_two_integers/SolutionTest.java

use leetcode_in_rust::s2235::add_two_integers::Solution;

#[test]
fn test_sum() {
    assert_eq!(Solution::sum(12, 5), 17);
}

#[test]
fn test_sum2() {
    assert_eq!(Solution::sum(-10, 4), -6);
}
