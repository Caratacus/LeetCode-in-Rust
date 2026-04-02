// Tests for Problem 0310: Minimum Height Trees
// Java reference: src/test/java/g0301_0400/s0310_minimum_height_trees/SolutionTest.java

use leetcode_in_rust::s0310::minimum_height_trees::Solution;

#[test]
fn test_find_min_height_trees() {
    let mut result = Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]);
    result.sort();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_find_min_height_trees2() {
    let mut result = Solution::find_min_height_trees(6, vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]);
    result.sort();
    assert_eq!(result, vec![3, 4]);
}
