// Tests for Problem 0827: Making A Large Island
// Java reference: src/test/java/g0801_0900/s0827_making_a_large_island/SolutionTest.java

use leetcode_in_rust::s0827::making_a_large_island::Solution;

#[test]
fn test_largest_island() {
    assert_eq!(Solution::largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
}

#[test]
fn test_largest_island2() {
    assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
}

#[test]
fn test_largest_island3() {
    assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
}
