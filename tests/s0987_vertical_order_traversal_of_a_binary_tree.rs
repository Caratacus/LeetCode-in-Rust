// Tests for Problem 0987: Vertical Order Traversal of a Binary Tree
// Java reference: src/test/java/g0901_1000/s0987_vertical_order_traversal_of_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s0987::vertical_order_traversal_of_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_vertical_traversal() {
    let root = tree_from_vec(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);
    assert_eq!(
        Solution::vertical_traversal(root),
        vec![vec![9], vec![3, 15], vec![20], vec![7]]
    );
}

#[test]
fn test_vertical_traversal2() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ]);
    assert_eq!(
        Solution::vertical_traversal(root),
        vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]
    );
}
