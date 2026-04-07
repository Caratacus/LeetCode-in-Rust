// Tests for Problem 2471: Minimum Number of Operations to Sort a Binary Tree by Level
// Java reference: src/test/java/g2401_2500/s2471_minimum_number_of_operations_to_sort_a_binary_tree_by_level/SolutionTest.java

use leetcode_in_rust::s2471::minimum_number_of_operations_to_sort_a_binary_tree_by_level::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_minimum_operations() {
    let root = tree_from_vec(vec![
        Some(1), Some(4), Some(3), Some(7), Some(6), Some(8), Some(5), None, None, None, None, Some(9), None, Some(10),
    ]);
    assert_eq!(Solution::minimum_operations(root), 3);
}

#[test]
fn test_minimum_operations2() {
    let root = tree_from_vec(vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4)]);
    assert_eq!(Solution::minimum_operations(root), 3);
}

#[test]
fn test_minimum_operations3() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(Solution::minimum_operations(root), 0);
}
