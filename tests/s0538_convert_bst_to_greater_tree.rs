// Tests for Problem 0538: Convert BST to Greater Tree
// Java reference: src/test/java/g0501_0600/s0538_convert_bst_to_greater_tree/SolutionTest.java

use leetcode_in_rust::s0538::convert_bst_to_greater_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_convert_bst() {
    let root = tree_from_vec(vec![
        Some(4), Some(1), Some(6), Some(0), Some(2), Some(5), Some(7),
        None, None, None, Some(3), None, None, None, Some(8)
    ]);
    let result = Solution::convert_bst(root);
    assert!(result.is_some());
}

#[test]
fn test_convert_bst2() {
    let root = tree_from_vec(vec![Some(0), None, Some(1)]);
    let result = Solution::convert_bst(root);
    assert!(result.is_some());
}
