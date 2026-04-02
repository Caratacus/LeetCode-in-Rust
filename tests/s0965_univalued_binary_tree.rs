// Tests for Problem 0965: Univalued Binary Tree
// Java reference: src/test/java/g0901_1000/s0965_univalued_binary_tree/SolutionTest.java

use leetcode_in_rust::s0965::univalued_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_unival_tree() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(1),
        Some(1),
        Some(1),
        Some(1),
        None,
        Some(1),
    ]);
    assert_eq!(Solution::is_unival_tree(root), true);
}

#[test]
fn test_is_unival_tree2() {
    let root = tree_from_vec(vec![Some(2), Some(2), Some(2), Some(5), Some(2)]);
    assert_eq!(Solution::is_unival_tree(root), false);
}
