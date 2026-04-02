// Tests for Problem 0998: Maximum Binary Tree II
// Java reference: src/test/java/g0901_1000/s0998_maximum_binary_tree_ii/SolutionTest.java

use leetcode_in_rust::s0998::maximum_binary_tree_ii::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_insert_into_max_tree() {
    let root = tree_from_vec(vec![Some(4), Some(1), Some(3), None, None, Some(2)]);
    let result = Solution::insert_into_max_tree(root, 5);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(5), Some(4), Some(1), Some(3), Some(2)]
    );
}

#[test]
fn test_insert_into_max_tree2() {
    let root = tree_from_vec(vec![Some(5), Some(2), Some(4), None, Some(1)]);
    let result = Solution::insert_into_max_tree(root, 3);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(5), Some(2), Some(1), Some(4), Some(3)]
    );
}

#[test]
fn test_insert_into_max_tree3() {
    let root = tree_from_vec(vec![Some(5), Some(2), Some(3), None, Some(1)]);
    let result = Solution::insert_into_max_tree(root, 4);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(5), Some(2), Some(1), Some(4), Some(3)]
    );
}
