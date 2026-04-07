// Tests for Problem 2641: Cousins in Binary Tree II
// Java reference: src/test/java/g2601_2700/s2641_cousins_in_binary_tree_ii/SolutionTest.java

use leetcode_in_rust::s2641::cousins_in_binary_tree_ii::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;
use leetcode_in_rust::utils::tree_utils::tree_to_vec;

#[test]
fn test_replace_value_in_tree() {
    let root = tree_from_vec(vec![
        Some(5),
        Some(4),
        Some(9),
        Some(1),
        Some(10),
        None,
        Some(7),
    ]);
    let result = Solution::replace_value_in_tree(root);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(0), Some(0), Some(0), Some(7), Some(7), None, Some(11)]
    );
}

#[test]
fn test_replace_value_in_tree2() {
    let root = tree_from_vec(vec![Some(3), Some(1), Some(2)]);
    let result = Solution::replace_value_in_tree(root);
    assert_eq!(tree_to_vec(result), vec![Some(0), Some(0), Some(0)]);
}
