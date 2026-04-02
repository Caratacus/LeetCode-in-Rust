// Tests for Problem 0173: Binary Search Tree Iterator
// Java reference: src/test/java/g0121_0200/s0173_binary_search_tree_iterator/SolutionTest.java

use leetcode_in_rust::s0173::binary_search_tree_iterator::BSTIterator;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_bst_iterator() {
    let root = tree_from_vec(vec![Some(7), Some(3), Some(15), None, None, Some(9), Some(20)]);
    let mut iter = BSTIterator::new(root);
    assert_eq!(iter.next(), 3);
    assert_eq!(iter.next(), 7);
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), 9);
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), 15);
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), 20);
    assert_eq!(iter.has_next(), false);
}
