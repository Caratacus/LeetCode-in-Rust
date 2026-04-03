// Tests for Problem 1530: Number of Good Leaf Nodes Pairs
// Java reference: src/test/java/g1501_1600/s1530_number_of_good_leaf_nodes_pairs/SolutionTest.java

use leetcode_in_rust::s1530::number_of_good_leaf_nodes_pairs::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_count_pairs() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(4)]);
    assert_eq!(Solution::count_pairs(root, 3), 1);
}

#[test]
fn test_count_pairs2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
    assert_eq!(Solution::count_pairs(root, 3), 2);
}

#[test]
fn test_count_pairs3() {
    let root = tree_from_vec(vec![
        Some(7), Some(1), Some(4), Some(6), None, Some(5), Some(3),
        None, None, None, None, None, Some(2)
    ]);
    assert_eq!(Solution::count_pairs(root, 3), 1);
}
