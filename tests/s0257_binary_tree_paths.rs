// Tests for Problem 0257: Binary Tree Paths
// Java reference: src/test/java/g0201_0300/s0257_binary_tree_paths/SolutionTest.java

use leetcode_in_rust::s0257::binary_tree_paths::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_binary_tree_paths() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(5)]);
    let mut result = Solution::binary_tree_paths(root);
    result.sort();
    let mut expected = vec!["1->2->5".to_string(), "1->3".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_binary_tree_paths2() {
    let root = tree_from_vec(vec![Some(1)]);
    let result = Solution::binary_tree_paths(root);
    assert_eq!(result, vec!["1".to_string()]);
}
