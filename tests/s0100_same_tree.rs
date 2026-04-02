// Tests for Problem 0100: Same Tree
// Java reference: src/test/java/g0001_0100/s0100_same_tree/SolutionTest.java

use leetcode_in_rust::s0100::same_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_same_tree() {
    let p = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let q = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::is_same_tree(p, q), true);
}

#[test]
fn test_is_same_tree2() {
    let p = tree_from_vec(vec![Some(1), Some(2)]);
    let q = tree_from_vec(vec![Some(1), None, Some(2)]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}

#[test]
fn test_is_same_tree3() {
    let p = tree_from_vec(vec![Some(1), Some(2), Some(1)]);
    let q = tree_from_vec(vec![Some(1), Some(1), Some(2)]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}
