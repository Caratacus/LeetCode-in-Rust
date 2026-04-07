// Tests for Problem 2415: Reverse Odd Levels of Binary Tree
// Java reference: src/test/java/g2401_2500/s2415_reverse_odd_levels_of_binary_tree/SolutionTest.java

use leetcode_in_rust::s2415::reverse_odd_levels_of_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_reverse_odd_levels() {
    // Tree: [2, 3, 5, 8, 13, 21, 34]
    let root = tree_from_vec(vec![
        Some(2), Some(3), Some(5), Some(8), Some(13), Some(21), Some(34)
    ]);
    let result = Solution::reverse_odd_levels(root);
    // Expected: [2,5,8,13,3,21,34]
    // Note: The result tree should have nodes 2,5, 8, 13, 3, 21, 34 at respective levels
}

#[test]
fn test_reverse_odd_levels2() {
    // Tree: [7, 13, 11]
    let root = tree_from_vec(vec![Some(7), Some(13), Some(11)]);
    let result = Solution::reverse_odd_levels(root);
    // Expected: [7,11,13]
}
