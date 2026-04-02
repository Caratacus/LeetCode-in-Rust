// Tests for Problem 0230: Kth Smallest Element in a BST
// Java reference: src/test/java/g0201_0300/s0230_kth_smallest_element_in_a_bst/SolutionTest.java

use leetcode_in_rust::s0230::kth_smallest_element_in_a_bst::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_kth_smallest() {
    let root = tree_from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]);
    assert_eq!(Solution::kth_smallest(root, 1), 1);
}

#[test]
fn test_kth_smallest2() {
    let root = tree_from_vec(vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)]);
    assert_eq!(Solution::kth_smallest(root, 3), 3);
}
