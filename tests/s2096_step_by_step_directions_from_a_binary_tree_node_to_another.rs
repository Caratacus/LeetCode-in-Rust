// Tests for Problem 2096: Step-By-Step Directions From a Binary Tree Node to Another
// Java reference: src/test/java/g2001_2100/s2096_step_by_step_directions_from_a_binary_tree_node_to_another/SolutionTest.java

use leetcode_in_rust::s2096::step_by_step_directions_from_a_binary_tree_node_to_another::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_get_directions() {
    let root = tree_from_vec(vec![Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)]);
    assert_eq!(Solution::get_directions(root, 3, 6), "UURL");
}

#[test]
fn test_get_directions2() {
    let root = tree_from_vec(vec![Some(2), Some(1)]);
    assert_eq!(Solution::get_directions(root, 2, 1), "L");
}
