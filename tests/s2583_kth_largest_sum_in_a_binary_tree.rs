// Tests for Problem 2583: Kth Largest Sum in a Binary Tree
// Java reference: src/test/java/g2501_2600/s2583_kth_largest_sum_in_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s2583::kth_largest_sum_in_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_kth_largest_level_sum() {
    let root = tree_from_vec(vec![Some(5), Some(8), Some(9), Some(2), Some(1), Some(3), Some(7), Some(4), Some(6)]);
    assert_eq!(Solution::kth_largest_level_sum(root, 2), 13);
}

#[test]
fn test_kth_largest_level_sum2() {
    let root = tree_from_vec(vec![Some(1), Some(2), None, Some(3)]);
    assert_eq!(Solution::kth_largest_level_sum(root, 1), 3);
}
