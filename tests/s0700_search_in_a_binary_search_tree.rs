// Tests for Problem 0700: Search in a Binary Search Tree
// Java reference: src/test/java/g0601_0700/s0700_search_in_a_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0700::search_in_a_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_search_bst() {
    let root = tree_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
    let result = Solution::search_bst(root, 2);
    // The result should be subtree rooted at 2 with children 1 and 3
    assert!(result.is_some());
    assert_eq!(result.unwrap().borrow().val, 2);
}

#[test]
fn test_search_bst2() {
    let root = tree_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
    let result = Solution::search_bst(root, 5);
    assert!(result.is_none());
}
