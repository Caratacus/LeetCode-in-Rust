// Tests for Problem 0733: Flood Fill
// Java reference: src/test/java/g0701_0800/s0733_flood_fill/SolutionTest.java

use leetcode_in_rust::s0733::flood_fill::Solution;

#[test]
fn test_flood_fill() {
    assert_eq!(
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );
}

#[test]
fn test_flood_fill2() {
    assert_eq!(
        Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 2]]
    );
}
