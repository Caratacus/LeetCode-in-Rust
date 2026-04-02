// Tests for Problem 0145: Binary Tree Postorder Traversal
// Java reference: src/test/java/g0121_0200/s0145_binary_tree_postorder_traversal/SolutionTest.java

use leetcode_in_rust::s0145::binary_tree_postorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_postorder_traversal() {
    let root = tree_from_vec(vec![Some(1), None, Some(2), Some(3)]);
    assert_eq!(Solution::postorder_traversal(root), vec![3, 2, 1]);
}

#[test]
fn test_postorder_traversal2() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::postorder_traversal(root), vec![] as Vec<i32>);
}

#[test]
fn test_postorder_traversal3() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::postorder_traversal(root), vec![1]);
}
