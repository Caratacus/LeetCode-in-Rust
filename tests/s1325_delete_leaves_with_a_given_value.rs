// Tests for Problem 1325: Delete Leaves With a Given Value
// Java reference: src/test/java/g1301_1400/s1325_delete_leaves_with_a_given_value/SolutionTest.java

use leetcode_in_rust::s1325::delete_leaves_with_a_given_value::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_remove_leaf_nodes() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)]);
    let result = Solution::remove_leaf_nodes(root, 2);
    let expected = tree_to_vec(result);
    assert_eq!(expected, vec![Some(1), None, Some(3), None, Some(4)]);
}

#[test]
fn test_remove_leaf_nodes2() {
    let root = tree_from_vec(vec![Some(1), Some(3), Some(3), Some(3), Some(2)]);
    let result = Solution::remove_leaf_nodes(root, 3);
    let expected = tree_to_vec(result);
    assert_eq!(expected, vec![Some(1), Some(3), None, None, Some(2)]);
}

#[test]
fn test_remove_leaf_nodes3() {
    let root = tree_from_vec(vec![Some(1), Some(2), None, Some(2), None, Some(2)]);
    let result = Solution::remove_leaf_nodes(root, 2);
    let expected = tree_to_vec(result);
    assert_eq!(expected, vec![Some(1)]);
}
