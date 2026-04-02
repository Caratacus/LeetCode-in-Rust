// Tests for Problem 0543: Diameter of Binary Tree
// Java reference: src/test/java/g0501_0600/s0543_diameter_of_binary_tree/SolutionTest.java

use leetcode_in_rust::s0543::diameter_of_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_diameter_of_binary_tree() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);
}

#[test]
fn test_diameter_of_binary_tree2() {
    let root = tree_from_vec(vec![Some(1), Some(2)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 1);
}
