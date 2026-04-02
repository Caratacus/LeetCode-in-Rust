// Tests for Problem 0427: Construct Quad Tree
// Java reference: src/test/java/g0401_0500/s0427_construct_quad_tree/SolutionTest.java

use leetcode_in_rust::s0427::construct_quad_tree::Solution;

#[test]
fn test_construct() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    let result = Solution::construct(grid);
    // Result should not be None
    assert!(result.is_some());
}

#[test]
fn test_construct2() {
    let grid = vec![vec![1, 1], vec![1, 1]];
    let result = Solution::construct(grid);
    // Result should not be None
    assert!(result.is_some());
}
