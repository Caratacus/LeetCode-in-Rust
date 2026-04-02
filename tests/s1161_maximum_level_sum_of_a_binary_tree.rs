// Tests for Problem 1161: Maximum Level Sum of a Binary Tree
// Java reference: src/test/java/g1101_1200/s1161_maximum_level_sum_of_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s1161::maximum_level_sum_of_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_max_level_sum() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(7),
        Some(0),
        Some(7),
        Some(-8),
        None,
        None,
    ]);
    assert_eq!(Solution::max_level_sum(root), 2);
}

#[test]
fn test_max_level_sum2() {
    let root = tree_from_vec(vec![
        Some(989),
        None,
        Some(10250),
        Some(98693),
        Some(-89388),
        None,
        None,
        None,
        Some(-32127),
    ]);
    assert_eq!(Solution::max_level_sum(root), 2);
}
