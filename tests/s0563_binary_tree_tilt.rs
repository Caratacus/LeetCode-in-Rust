// Tests for Problem 0563: Binary Tree Tilt
// Java reference: src/test/java/g0501_0600/s0563_binary_tree_tilt/SolutionTest.java

use leetcode_in_rust::s0563::binary_tree_tilt::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_tilt() {
    // Tree: [1, 2, 3]
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::find_tilt(root), 1);
}

#[test]
fn test_find_tilt2() {
    // Tree: [4, 2, 9, 3, 5, null, 7]
    let root = tree_from_vec(vec![Some(4), Some(2), Some(9), Some(3), Some(5), None, Some(7)]);
    assert_eq!(Solution::find_tilt(root), 15);
}

#[test]
fn test_find_tilt3() {
    // Tree: [21, 7, 14, 1, 1, 2, 2, 3, 3]
    let root = tree_from_vec(vec![
        Some(21), Some(7), Some(14), Some(1), Some(1), Some(2), Some(2), Some(3), Some(3),
    ]);
    assert_eq!(Solution::find_tilt(root), 9);
}
