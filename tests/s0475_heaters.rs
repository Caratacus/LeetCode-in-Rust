// Tests for Problem 0475: Heaters
// Java reference: src/test/java/g0401_0500/s0475_heaters/SolutionTest.java

use leetcode_in_rust::s0475::heaters::Solution;

#[test]
fn test_find_radius() {
    assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![2]), 1);
}

#[test]
fn test_find_radius2() {
    assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
}

#[test]
fn test_find_radius3() {
    assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
}
