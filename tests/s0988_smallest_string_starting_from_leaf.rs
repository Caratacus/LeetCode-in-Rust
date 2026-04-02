// Tests for Problem 0988: Smallest String Starting From Leaf
// Java reference: src/test/java/g0901_1000/s0988_smallest_string_starting_from_leaf/SolutionTest.java

use leetcode_in_rust::s0988::smallest_string_starting_from_leaf::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_smallest_from_leaf() {
    let root = tree_from_vec(vec![
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(3),
        Some(4),
    ]);
    assert_eq!(Solution::smallest_from_leaf(root), "dba");
}

#[test]
fn test_smallest_from_leaf2() {
    let root = tree_from_vec(vec![
        Some(25),
        Some(1),
        Some(3),
        Some(1),
        Some(3),
        Some(0),
        Some(2),
    ]);
    assert_eq!(Solution::smallest_from_leaf(root), "adz");
}

#[test]
fn test_smallest_from_leaf3() {
    let root = tree_from_vec(vec![
        Some(2),
        Some(2),
        Some(1),
        None,
        Some(1),
        Some(0),
        None,
        Some(0),
    ]);
    assert_eq!(Solution::smallest_from_leaf(root), "abc");
}
