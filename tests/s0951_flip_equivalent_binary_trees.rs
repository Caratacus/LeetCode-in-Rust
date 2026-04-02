// Tests for Problem 0951: Flip Equivalent Binary Trees
// Java reference: src/test/java/g0901_1000/s0951_flip_equivalent_binary_trees/SolutionTest.java

use leetcode_in_rust::s0951::flip_equivalent_binary_trees::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_flip_equiv() {
    let root1 = tree_from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        Some(7),
        Some(8),
    ]);
    let root2 = tree_from_vec(vec![
        Some(1),
        Some(3),
        Some(2),
        None,
        Some(6),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        Some(8),
        Some(7),
    ]);
    assert_eq!(Solution::flip_equiv(root1, root2), true);
}

#[test]
fn test_flip_equiv2() {
    let root1 = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let root2 = tree_from_vec(vec![Some(1), Some(2), Some(4)]);
    assert_eq!(Solution::flip_equiv(root1, root2), false);
}
