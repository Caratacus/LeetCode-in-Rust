// Tests for Problem 0863: All Nodes Distance K in Binary Tree
// Java reference: src/test/java/g0801_0900/s0863_all_nodes_distance_k_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s0863::all_nodes_distance_k_in_binary_tree::Solution;
use leetcode_in_rust::common::tree_node::TreeNode;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_distance_k() {
    let root = tree_from_vec(vec![
        Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8),
        None, None, Some(7), Some(4)
    ]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let mut result = Solution::distance_k(root, target, 2);
    result.sort();
    assert_eq!(result, vec![1, 4, 7]);
}

#[test]
fn test_distance_k2() {
    let root = tree_from_vec(vec![Some(1)]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::distance_k(root, target, 3), Vec::<i32>::new());
}

#[test]
fn test_distance_k3() {
    let root = tree_from_vec(vec![
        Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8),
        None, None, Some(7), Some(4)
    ]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut result = Solution::distance_k(root, target, 2);
    result.sort();
    assert_eq!(result, vec![0, 2, 6, 8]);
}

#[test]
fn test_distance_k4() {
    let root = tree_from_vec(vec![
        Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8),
        None, None, Some(7), Some(4)
    ]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    assert_eq!(Solution::distance_k(root, target, 1), vec![2]);
}

#[test]
fn test_distance_k5() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(Solution::distance_k(root, target, 0), vec![2]);
}

#[test]
fn test_distance_k6() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::distance_k(root, target, 5), Vec::<i32>::new());
}

#[test]
fn test_distance_k7() {
    let root = tree_from_vec(vec![Some(1), Some(2), None, Some(3), None, Some(4)]);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(Solution::distance_k(root, target, 2), vec![4]);
}
