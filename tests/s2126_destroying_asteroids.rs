// Tests for Problem 2126: Destroying Asteroids
// Java reference: src/test/java/g2101_2200/s2126_destroying_asteroids/SolutionTest.java

use leetcode_in_rust::s2126::destroying_asteroids::Solution;

#[test]
fn test_asteroids_destroyed() {
    assert_eq!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]), true);
}

#[test]
fn test_asteroids_destroyed2() {
    assert_eq!(Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]), false);
}
