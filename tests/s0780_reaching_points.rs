// Tests for Problem 0780: Reaching Points
// Java reference: src/test/java/g0701_0800/s0780_reaching_points/SolutionTest.java

use leetcode_in_rust::s0780::reaching_points::Solution;

#[test]
fn test_reaching_points() {
    assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
}

#[test]
fn test_reaching_points2() {
    assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
}

#[test]
fn test_reaching_points3() {
    assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
}
