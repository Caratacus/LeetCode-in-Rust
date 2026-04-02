// Tests for Problem 1022: Sum of Root To Leaf Binary Numbers
// Java reference: src/test/java/g1001_1100/s1022_sum_of_root_to_leaf_binary_numbers/SolutionTest.java

use leetcode_in_rust::s1022::sum_of_root_to_leaf_binary_numbers::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_sum_root_to_leaf() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(0),
        Some(1),
        Some(0),
        Some(1),
        Some(0),
        Some(1),
    ]);
    assert_eq!(Solution::sum_root_to_leaf(root), 22);
}

#[test]
fn test_sum_root_to_leaf2() {
    let root = tree_from_vec(vec![Some(0)]);
    assert_eq!(Solution::sum_root_to_leaf(root), 0);
}
