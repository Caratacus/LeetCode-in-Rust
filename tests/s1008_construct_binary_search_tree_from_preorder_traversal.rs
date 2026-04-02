// Tests for Problem 1008: Construct Binary Search Tree From Preorder Traversal
// Java reference: src/test/java/g1001_1100/s1008_construct_binary_search_tree_from_preorder_traversal/SolutionTest.java

use leetcode_in_rust::s1008::construct_binary_search_tree_from_preorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_to_vec;

#[test]
fn test_bst_from_preorder() {
    let result = Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)]
    );
}

#[test]
fn test_bst_from_preorder2() {
    let result = Solution::bst_from_preorder(vec![1, 3]);
    assert_eq!(tree_to_vec(result), vec![Some(1), None, Some(3)]);
}
