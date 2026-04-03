// Tests for Problem 1448: Count Good Nodes in Binary Tree
// Java reference: src/test/java/g1401_1500/s1448_count_good_nodes_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s1448::count_good_nodes_in_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_good_nodes() {
    let root = tree_from_vec(vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)]);
    assert_eq!(Solution::good_nodes(root), 4);
}

#[test]
fn test_good_nodes2() {
    let root = tree_from_vec(vec![Some(3), Some(3), None, Some(4), Some(2)]);
    assert_eq!(Solution::good_nodes(root), 3);
}

#[test]
fn test_good_nodes3() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::good_nodes(root), 1);
}
