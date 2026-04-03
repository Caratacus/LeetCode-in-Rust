// Tests for Problem 1361: Validate Binary Tree Nodes
// Java reference: src/test/java/g1301_1400/s1361_validate_binary_tree_nodes/SolutionTest.java

use leetcode_in_rust::s1361::validate_binary_tree_nodes::Solution;

#[test]
fn test_validate_binary_tree_nodes() {
    assert_eq!(
        Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]),
        true
    );
}

#[test]
fn test_validate_binary_tree_nodes2() {
    assert_eq!(
        Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]),
        false
    );
}

#[test]
fn test_validate_binary_tree_nodes3() {
    assert_eq!(
        Solution::validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]),
        false
    );
}
