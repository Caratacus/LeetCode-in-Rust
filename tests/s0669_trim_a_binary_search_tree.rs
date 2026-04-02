// Tests for Problem 0669: Trim a Binary Search Tree
// Java reference: src/test/java/g0601_0700/s0669_trim_a_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0669::trim_a_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_trim_bst() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 0, 2));
    let tree = tree_from_vec(vec![Some(1), Some(0), Some(2)]);

    // TreeNode expected = TreeNode.create(Arrays.asList(1, null, 2));
    let result = Solution::trim_bst(tree, 1, 2);
    let result_vec = tree_to_vec(result);

    assert_eq!(result_vec, vec![Some(1), None, Some(2)]);
}

#[test]
fn test_trim_bst2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(3, 0, 4, null, 2, null, null, 1));
    let tree = tree_from_vec(vec![
        Some(3),
        Some(0),
        Some(4),
        None,
        Some(2),
        None,
        None,
        Some(1),
    ]);

    // TreeNode expected = TreeNode.create(Arrays.asList(3, 2, null, 1));
    let result = Solution::trim_bst(tree, 1, 3);
    let result_vec = tree_to_vec(result);

    assert_eq!(result_vec, vec![Some(3), Some(2), None, Some(1)]);
}
