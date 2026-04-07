// Tests for Problem 1932: Merge BSTs to Create Single BST
// Java reference: src/test/java/g1901_2000/s1932_merge_bsts_to_create_single_bst/SolutionTest.java

use leetcode_in_rust::s1932::merge_bsts_to_create_single_bst::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_can_merge() {
    let trees = vec![
        tree_from_vec(vec![Some(2), Some(1)]),
        tree_from_vec(vec![Some(3), Some(2), Some(5)]),
        tree_from_vec(vec![Some(5), Some(4)]),
    ];
    let result = Solution::can_merge(trees);
    let expected = tree_from_vec(vec![Some(3), Some(2), Some(5), Some(1), None, Some(4)]);
    assert_eq!(tree_to_vec(result), tree_to_vec(expected));
}

#[test]
fn test_can_merge2() {
    let trees = vec![
        tree_from_vec(vec![Some(5), Some(3), Some(8)]),
        tree_from_vec(vec![Some(3), Some(2), Some(6)]),
    ];
    let result = Solution::can_merge(trees);
    assert!(result.is_none());
}

#[test]
fn test_can_merge3() {
    let trees = vec![
        tree_from_vec(vec![Some(5), Some(4)]),
        tree_from_vec(vec![Some(3)]),
    ];
    let result = Solution::can_merge(trees);
    assert!(result.is_none());
}
