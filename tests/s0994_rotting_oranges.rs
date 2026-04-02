// Tests for Problem 0994: Rotting Oranges
// Java reference: src/test/java/g0901_1000/s0994_rotting_oranges/SolutionTest.java

use leetcode_in_rust::s0994::rotting_oranges::Solution;

#[test]
fn test_oranges_rotting() {
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
        4
    );
}

#[test]
fn test_oranges_rotting2() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
}
