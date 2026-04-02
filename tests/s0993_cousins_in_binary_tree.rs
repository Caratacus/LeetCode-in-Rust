// Tests for Problem 0993: Cousins in Binary Tree
// Java reference: src/test/java/g0901_1000/s0993_cousins_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s0993::cousins_in_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_cousins() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4)]);
    assert_eq!(Solution::is_cousins(root, 4, 3), false);
}

#[test]
fn test_is_cousins2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(4), None, Some(5)]);
    assert_eq!(Solution::is_cousins(root, 5, 4), true);
}

#[test]
fn test_is_cousins3() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(4)]);
    assert_eq!(Solution::is_cousins(root, 2, 3), false);
}
