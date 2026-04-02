// Tests for Problem 0101: Symmetric Tree
// Java reference: src/test/java/g0101_0200/s0101_symmetric_tree/SolutionTest.java

use leetcode_in_rust::s0101::symmetric_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_symmetric() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]);
    assert_eq!(Solution::is_symmetric(root), true);
}

#[test]
fn test_is_symmetric2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);
    assert_eq!(Solution::is_symmetric(root), false);
}

#[test]
fn test_is_symmetric3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::is_symmetric(root), true);
}

#[test]
fn test_is_symmetric4() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::is_symmetric(root), true);
}

#[test]
fn test_is_symmetric5() {
    let root = tree_from_vec(vec![Some(1), Some(2), None]);
    assert_eq!(Solution::is_symmetric(root), false);
}

#[test]
fn test_is_symmetric6() {
    let root = tree_from_vec(vec![Some(1), None, Some(2)]);
    assert_eq!(Solution::is_symmetric(root), false);
}

#[test]
fn test_is_symmetric7() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(2), Some(3), Some(4), Some(5), Some(3)]);
    assert_eq!(Solution::is_symmetric(root), false);
}
