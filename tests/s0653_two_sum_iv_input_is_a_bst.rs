// Tests for Problem 0653: Two Sum IV - Input is a BST
// Java reference: src/test/java/g0601_0700/s0653_two_sum_iv_input_is_a_bst/SolutionTest.java

use leetcode_in_rust::s0653::two_sum_iv_input_is_a_bst::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_target() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(5, 3, 6, 2, 4, null, 7));
    let tree = tree_from_vec(vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    assert_eq!(Solution::find_target(tree, 9), true);
}

#[test]
fn test_find_target2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(5, 3, 6, 2, 4, null, 7));
    let tree = tree_from_vec(vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    assert_eq!(Solution::find_target(tree, 28), false);
}
