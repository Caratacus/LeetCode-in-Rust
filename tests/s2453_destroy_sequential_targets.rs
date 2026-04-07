// Tests for Problem 2453: Destroy Sequential Targets
// Java reference: src/test/java/g2401_2500/s2453_destroy_sequential_targets/SolutionTest.java

use leetcode_in_rust::s2453::destroy_sequential_targets::Solution;

#[test]
fn test_destroy_targets() {
    assert_eq!(Solution::destroy_targets(vec![3, 7, 8, 1, 1, 5], 2), 1);
}

#[test]
fn test_destroy_targets2() {
    assert_eq!(Solution::destroy_targets(vec![1, 3, 5, 2, 4, 6], 2), 1);
}

#[test]
fn test_destroy_targets3() {
    assert_eq!(Solution::destroy_targets(vec![6, 2, 5], 100), 2);
}
