// Tests for Problem 0096: Unique Binary Search Trees
// Java reference: src/test/java/g0001_0100/s0096_unique_binary_search_trees/SolutionTest.java

use leetcode_in_rust::s0096::unique_binary_search_trees::Solution;

#[test]
fn test_num_trees() {
    assert_eq!(Solution::num_trees(3), 5);
}

#[test]
fn test_num_trees2() {
    assert_eq!(Solution::num_trees(1), 1);
}

#[test]
fn test_num_trees3() {
    assert_eq!(Solution::num_trees(4), 14);
}
