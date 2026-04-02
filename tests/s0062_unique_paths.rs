// Tests for Problem 0062: Unique Paths
// Java reference: src/test/java/g0001_0100/s0062_unique_paths/SolutionTest.java

use leetcode_in_rust::s0062::unique_paths::Solution;

#[test]
fn test_unique_paths() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
}

#[test]
fn test_unique_paths2() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
}
