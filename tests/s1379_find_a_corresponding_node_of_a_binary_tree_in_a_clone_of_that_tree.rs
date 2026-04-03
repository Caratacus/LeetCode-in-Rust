// Tests for Problem 1379: Find a Corresponding Node of a Binary Tree in a Clone of That Tree
// Java reference: src/test/java/g1301_1400/s1379_find_a_corresponding_node_of_a_binary_tree_in_a_clone_of_that_tree/SolutionTest.java

use leetcode_in_rust::s1379::find_a_corresponding_node_of_a_binary_tree_in_a_clone_of_that_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_get_target_copy() {
    // TreeNode original = TreeNode.create(Arrays.asList(7, 4, 3, null, null, 6, 19));
    // TreeNode cloned = TreeNode.create(Arrays.asList(7, 4, 3, null, null, 6, 19));
    // TreeNode target = original.as_ref().unwrap().borrow().right.as_ref().unwrap().clone();
    let original = tree_from_vec(vec![Some(7), Some(4), Some(3), None, None, Some(6), Some(19)]);
    let cloned = tree_from_vec(vec![Some(7), Some(4), Some(3), None, None, Some(6), Some(19)]);
    // Get reference to node with value 3 (right child of root)
    let target = original.as_ref().and_then(|n| n.borrow().right.clone());
    let result = Solution::get_target_copy(original, cloned, target);
    assert_eq!(result.as_ref().map(|n| n.borrow().val), Some(3));
}

#[test]
fn test_get_target_copy2() {
    // TreeNode original = TreeNode.create(Arrays.asList(7));
    // TreeNode cloned = TreeNode.create(Arrays.asList(7));
    // TreeNode target = original.clone();
    let original = tree_from_vec(vec![Some(7)]);
    let cloned = tree_from_vec(vec![Some(7)]);
    let target = original.clone();
    let result = Solution::get_target_copy(original, cloned, target);
    assert_eq!(result.as_ref().map(|n| n.borrow().val), Some(7));
}

#[test]
fn test_get_target_copy3() {
    // TreeNode original = TreeNode.create(Arrays.asList(8, null, 6, null, 5, null, 4, null, 3, null, 2, null, 1));
    // TreeNode cloned = TreeNode.create(Arrays.asList(8, null, 6, null, 5, null, 4, null, 3, null, 2, null, 1));
    // TreeNode target = ... node with value 4
    let original = tree_from_vec(vec![
        Some(8), None, Some(6), None, Some(5), None, Some(4),
        None, Some(3), None, Some(2), None, Some(1)
    ]);
    let cloned = tree_from_vec(vec![
        Some(8), None, Some(6), None, Some(5), None, Some(4),
        None, Some(3), None, Some(2), None, Some(1)
    ]);
    // Navigate to node with value 4 (right->right->right from root)
    let target = original.as_ref()
        .and_then(|n| n.borrow().right.clone())
        .and_then(|n| n.borrow().right.clone())
        .and_then(|n| n.borrow().right.clone());
    let result = Solution::get_target_copy(original, cloned, target);
    assert_eq!(result.as_ref().map(|n| n.borrow().val), Some(4));
}
