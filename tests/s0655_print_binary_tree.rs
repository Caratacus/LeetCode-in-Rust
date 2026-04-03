// Tests for Problem 0655: Print Binary Tree
// Java reference: src/test/java/g0601_0700/s0655_print_binary_tree/SolutionTest.java

use leetcode_in_rust::s0655::print_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_print_tree() {
    // Tree: [1, 2]
    let root = tree_from_vec(vec![Some(1), Some(2)]);
    let result = Solution::print_tree(root);
    // Expected: [["", "1", ""], ["2", "", ""]]
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].len(), 3);
    assert_eq!(result[0][1], "1");
}

#[test]
fn test_print_tree2() {
    // Tree: [1, 2, 3, null, 4]
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(4)]);
    let result = Solution::print_tree(root);
    // Expected: [["", "", "", "1", "", "", ""], ["", "2", "", "", "", "3", ""], ["", "", "4", "", "", "", ""]]
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].len(), 7);
    assert_eq!(result[0][3], "1");
}
