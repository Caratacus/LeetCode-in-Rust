// Tests for Problem 0894: All Possible Full Binary Trees
// Java reference: src/test/java/g0801_0900/s0894_all_possible_full_binary_trees/SolutionTest.java

use leetcode_in_rust::s0894::all_possible_full_binary_trees::Solution;

#[test]
fn test_all_possible_fbt() {
    let result = Solution::all_possible_fbt(7);
    // There should be 5 possible full binary trees with 7 nodes
    assert_eq!(result.len(), 5);
}

#[test]
fn test_all_possible_fbt2() {
    let result = Solution::all_possible_fbt(3);
    // There should be 1 possible full binary tree with 3 nodes
    assert_eq!(result.len(), 1);
}
