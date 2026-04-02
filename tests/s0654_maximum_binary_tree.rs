// Tests for Problem 0654: Maximum Binary Tree
// Java reference: src/test/java/g0601_0700/s0654_maximum_binary_tree/SolutionTest.java

use leetcode_in_rust::s0654::maximum_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_construct_maximum_binary_tree() {
    // TreeNode expected = TreeNode.create(Arrays.asList(6, 3, 5, null, 2, 0, null, null, 1));
    let result = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
    let result_vec = tree_to_vec(result);
    assert_eq!(
        result_vec,
        vec![Some(6), Some(3), Some(5), None, Some(2), Some(0), None, None, Some(1)]
    );
}

#[test]
fn test_construct_maximum_binary_tree2() {
    // TreeNode expected = TreeNode.create(Arrays.asList(3, null, 2, null, 1));
    let result = Solution::construct_maximum_binary_tree(vec![3, 2, 1]);
    let result_vec = tree_to_vec(result);
    assert_eq!(result_vec, vec![Some(3), None, Some(2), None, Some(1)]);
}
