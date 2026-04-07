// Tests for Problem 2265: Count Nodes Equal to Average of Subtree
// Java reference: src/test/java/g2201_2300/s2265_count_nodes_equal_to_average_of_subtree/SolutionTest.java

use leetcode_in_rust::s2265::count_nodes_equal_to_average_of_subtree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_average_of_subtree() {
    let root = tree_from_vec(vec![Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)]);
    assert_eq!(Solution::average_of_subtree(root), 5);
}

#[test]
fn test_average_of_subtree2() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::average_of_subtree(root), 1);
}
