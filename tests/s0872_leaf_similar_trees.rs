// Tests for Problem 0872: Leaf Similar Trees
// Java reference: src/test/java/g0801_0900/s0872_leaf_similar_trees/SolutionTest.java

use leetcode_in_rust::s0872::leaf_similar_trees::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_leaf_similar() {
    let root1 = tree_from_vec(vec![Some(3), Some(5), Some(6), Some(2), Some(7), Some(4), Some(1), Some(9), Some(8)]);
    let root2 = tree_from_vec(vec![Some(3), Some(5), Some(6), Some(2), Some(7), Some(4), Some(1), Some(9), Some(8)]);
    assert_eq!(Solution::leaf_similar(root1, root2), true);
}

#[test]
fn test_leaf_similar2() {
    let root1 = tree_from_vec(vec![Some(3), Some(5), Some(1), Some(6), Some(2), None, None, Some(9), Some(8), None, None, Some(7), Some(4)]);
    let root2 = tree_from_vec(vec![Some(3), Some(5), Some(1), Some(6), Some(7), Some(4), Some(2), None, None, None, None, None, None, Some(9), Some(8)]);
    assert_eq!(Solution::leaf_similar(root1, root2), true);
}

#[test]
fn test_leaf_similar3() {
    let root1 = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let root2 = tree_from_vec(vec![Some(1), Some(3), Some(2)]);
    assert_eq!(Solution::leaf_similar(root1, root2), false);
}
