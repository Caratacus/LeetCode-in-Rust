// Tests for Problem 0971: Flip Binary Tree to Match Preorder Traversal
// Java reference: src/test/java/g0901_1000/s0971_flip_binary_tree_to_match_preorder_traversal/SolutionTest.java

use leetcode_in_rust::s0971::flip_binary_tree_to_match_preorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_flip_match_voyage() {
    let root = tree_from_vec(vec![Some(1), Some(2)]);
    let result = Solution::flip_match_voyage(root, vec![2, 1]);
    assert_eq!(result, vec![-1]);
}

#[test]
fn test_flip_match_voyage2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let result = Solution::flip_match_voyage(root, vec![1, 3, 2]);
    assert_eq!(result, vec![1]);
}

#[test]
fn test_flip_match_voyage3() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let result = Solution::flip_match_voyage(root, vec![1, 2, 3]);
    assert_eq!(result, vec![]);
}
