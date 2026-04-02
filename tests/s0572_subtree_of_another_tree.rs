// Tests for Problem 0572: Subtree of Another Tree
// Java reference: src/test/java/g0501_0600/s0572_subtree_of_another_tree/SolutionTest.java

use leetcode_in_rust::s0572::subtree_of_another_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_subtree() {
    let root = tree_from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
    let sub_root = tree_from_vec(vec![Some(4), Some(1), Some(2)]);
    assert_eq!(Solution::is_subtree(root, sub_root), true);
}

#[test]
fn test_is_subtree2() {
    let root = tree_from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, None, Some(0)]);
    let sub_root = tree_from_vec(vec![Some(4), Some(1), Some(2)]);
    assert_eq!(Solution::is_subtree(root, sub_root), false);
}
