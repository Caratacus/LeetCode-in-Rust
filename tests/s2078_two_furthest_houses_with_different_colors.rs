// Tests for Problem 2078: Two Furthest Houses With Different Colors
// Java reference: src/test/java/g2001_2100/s2078_two_furthest_houses_with_different_colors/SolutionTest.java

use leetcode_in_rust::s2078::two_furthest_houses_with_different_colors::Solution;

#[test]
fn test_max_distance() {
    assert_eq!(Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
}

#[test]
fn test_max_distance2() {
    assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
}

#[test]
fn test_max_distance3() {
    assert_eq!(Solution::max_distance(vec![0, 1]), 1);
}
