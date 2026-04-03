// Tests for Problem 1609: Even Odd Tree
// Java reference: src/test/java/g1601_1700/s1609_even_odd_tree/SolutionTest.java

use leetcode_in_rust::s1609::even_odd_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_even_odd_tree() {
    let tree = tree_from_vec(vec![
        Some(1),
        Some(10),
        Some(4),
        Some(3),
        None,
        Some(7),
        Some(9),
        Some(12),
        Some(8),
        Some(6),
        None,
        None,
        Some(2),
    ]);
    assert_eq!(Solution::is_even_odd_tree(tree), true);
}

#[test]
fn test_is_even_odd_tree2() {
    let tree = tree_from_vec(vec![Some(5), Some(4), Some(2), Some(3), Some(3), Some(7)]);
    assert_eq!(Solution::is_even_odd_tree(tree), false);
}

#[test]
fn test_is_even_odd_tree3() {
    let tree = tree_from_vec(vec![Some(5), Some(9), Some(1), Some(3), Some(5), Some(7)]);
    assert_eq!(Solution::is_even_odd_tree(tree), false);
}
