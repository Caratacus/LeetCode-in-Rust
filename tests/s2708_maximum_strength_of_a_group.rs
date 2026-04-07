// Tests for Problem 2708: Maximum Strength of a Group
// Java reference: src/test/java/g2701_2800/s2708_maximum_strength_of_a_group/SolutionTest.java

use leetcode_in_rust::s2708::maximum_strength_of_a_group::Solution;

#[test]
fn test_max_strength() {
    assert_eq!(Solution::max_strength(vec![3, -1, -5, 2, 5, -9]), 1350);
}

#[test]
fn test_max_strength2() {
    assert_eq!(Solution::max_strength(vec![-4, -5, -4]), 20);
}

#[test]
fn test_max_strength3() {
    assert_eq!(
        Solution::max_strength(vec![8, 6, 0, 5, -4, -8, -4, 9, -1, 6, -4, 8, -5]),
        265420800
    );
}
