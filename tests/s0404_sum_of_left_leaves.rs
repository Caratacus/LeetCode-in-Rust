// Tests for Problem 0404: Sum of Left Leaves
// Java reference: src/test/java/g0401_0500/s0404_sum_of_left_leaves/SolutionTest.java

use leetcode_in_rust::s0404::sum_of_left_leaves::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_sum_of_left_leaves() {
    let root = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::sum_of_left_leaves(root), 24);
}

#[test]
fn test_sum_of_left_leaves2() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::sum_of_left_leaves(root), 0);
}
