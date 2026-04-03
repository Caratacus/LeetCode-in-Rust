// Tests for Problem 1382: Balance a Binary Search Tree
// Java reference: src/test/java/g1301_1400/s1382_balance_a_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s1382::balance_a_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_balance_bst() {
    // TreeNode.create(Arrays.asList(1, null, 2, null, 3, null, 4, null, null))
    let tree = tree_from_vec(vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, None]);
    let result = Solution::balance_bst(tree);
    // Result should be a balanced BST with same elements
    let result_vec = tree_to_vec(result);
    // Check that it contains the same values (2, 1, 3, 4 in some balanced order)
    assert!(result_vec.contains(&Some(2)));
    assert!(result_vec.contains(&Some(1)));
    assert!(result_vec.contains(&Some(3)));
    assert!(result_vec.contains(&Some(4)));
}

#[test]
fn test_balance_bst2() {
    // TreeNode.create(Arrays.asList(2, 1, 3))
    let tree = tree_from_vec(vec![Some(2), Some(1), Some(3)]);
    let result = Solution::balance_bst(tree);
    let result_vec = tree_to_vec(result);
    // Already balanced, should remain 2, 1, 3
    assert_eq!(result_vec, vec![Some(2), Some(1), Some(3)]);
}
