// Tests for Problem 1457: Pseudo-Palindromic Paths in a Binary Tree
// Java reference: src/test/java/g1401_1500/s1457_pseudo_palindromic_paths_in_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s1457::pseudo_palindromic_paths_in_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_pseudo_palindromic_paths() {
    let root = tree_from_vec(vec![Some(2), Some(3), Some(1), Some(3), Some(1), None, Some(1)]);
    assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
}

#[test]
fn test_pseudo_palindromic_paths2() {
    let root = tree_from_vec(vec![Some(2), Some(1), Some(1), Some(1), Some(3), None, None, None, None, None, Some(1)]);
    assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
}

#[test]
fn test_pseudo_palindromic_paths3() {
    let root = tree_from_vec(vec![Some(9)]);
    assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
}
