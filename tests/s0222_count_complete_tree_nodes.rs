// Tests for Problem 0222: Count Complete Tree Nodes
// Java reference: src/test/java/g0201_0300/s0222_count_complete_tree_nodes/SolutionTest.java

use leetcode_in_rust::s0222::count_complete_tree_nodes::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_count_nodes() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(Solution::count_nodes(root), 6);
}

#[test]
fn test_count_nodes2() {
    assert_eq!(Solution::count_nodes(None), 0);
}

#[test]
fn test_count_nodes3() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::count_nodes(root), 1);
}
