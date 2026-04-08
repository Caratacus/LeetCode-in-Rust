// Tests for Problem 3319: K-th Largest Perfect Subtree Size in Binary Tree
// Java reference: src/test/java/g3301_3400/s3319_k_th_largest_perfect_subtree_size_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s3319::k_th_largest_perfect_subtree_size_in_binary_tree::Solution;
use leetcode_in_rust::common::tree_node::TreeNode;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_kth_largest_perfect_subtree() {
    let root = tree_from_vec(&vec![
        Some(5), Some(3), Some(6), Some(5), Some(2), Some(5), Some(7),
        Some(1), Some(8), None, None, Some(6), Some(8)
    ]);
    assert_eq!(Solution::kth_largest_perfect_subtree(root, 2), 3);
}

#[test]
fn test_kth_largest_perfect_subtree2() {
    let root = tree_from_vec(&vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
    assert_eq!(Solution::kth_largest_perfect_subtree(root, 1), 7);
}

#[test]
fn test_kth_largest_perfect_subtree3() {
    let root = tree_from_vec(&vec![Some(1), Some(2), Some(3), None, Some(4)]);
    assert_eq!(Solution::kth_largest_perfect_subtree(root, 3), -1);
}
