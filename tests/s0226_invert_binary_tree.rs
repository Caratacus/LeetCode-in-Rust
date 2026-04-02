// Tests for Problem 0226: Invert Binary Tree
// Java reference: src/test/java/g0201_0300/s0226_invert_binary_tree/SolutionTest.java

use leetcode_in_rust::s0226::invert_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_invert_tree() {
    let root = tree_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let result = Solution::invert_tree(root);
    assert_eq!(tree_to_vec(result), vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]);
}

#[test]
fn test_invert_tree2() {
    let root = tree_from_vec(vec![Some(2), Some(1), Some(3)]);
    let result = Solution::invert_tree(root);
    assert_eq!(tree_to_vec(result), vec![Some(2), Some(3), Some(1)]);
}
