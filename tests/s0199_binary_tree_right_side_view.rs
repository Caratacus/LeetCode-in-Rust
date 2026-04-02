// Tests for Problem 0199: Binary Tree Right Side View
// Java reference: src/test/java/g0181_0200/s0199_binary_tree_right_side_view/SolutionTest.java

use leetcode_in_rust::s0199::binary_tree_right_side_view::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_right_side_view() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
    assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
}

#[test]
fn test_right_side_view2() {
    let root = tree_from_vec(vec![Some(1), None, Some(3)]);
    assert_eq!(Solution::right_side_view(root), vec![1, 3]);
}

#[test]
fn test_right_side_view3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::right_side_view(root), vec![] as Vec<i32>);
}
