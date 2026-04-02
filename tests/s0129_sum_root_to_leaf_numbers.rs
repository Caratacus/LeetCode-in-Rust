// Tests for Problem 0129: Sum Root to Leaf Numbers
// Java reference: src/test/java/g0121_0200/s0129_sum_root_to_leaf_numbers/SolutionTest.java

use leetcode_in_rust::s0129::sum_root_to_leaf_numbers::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_sum_numbers() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::sum_numbers(root), 25);
}

#[test]
fn test_sum_numbers2() {
    let root = tree_from_vec(vec![Some(4), Some(9), Some(0), Some(5), Some(1)]);
    assert_eq!(Solution::sum_numbers(root), 1026);
}
