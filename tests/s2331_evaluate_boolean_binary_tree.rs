// Tests for Problem 2331: Evaluate Boolean Binary Tree
// Java reference: src/test/java/g2301_2400/s2331_evaluate_boolean_binary_tree/SolutionTest.java

use leetcode_in_rust::s2331::evaluate_boolean_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_evaluate_tree() {
    // Tree: [2, 1, 3, null, null, 0, 1]
    let root = tree_from_vec(vec![Some(2), Some(1), Some(3), None, None, Some(0), Some(1)]);
    assert_eq!(Solution::evaluate_tree(root), true);
}

#[test]
fn test_evaluate_tree2() {
    // Tree: [0]
    let root = tree_from_vec(vec![Some(0)]);
    assert_eq!(Solution::evaluate_tree(root), false);
}

#[test]
fn test_evaluate_tree3() {
    // Tree: [3, 2, 2, 2, 2, 3, 2, 0, 0, 0, 1, 1, 2, 1, 1, null, null, null, null, null, null, null,
    //        null, null, null, 3, 2, null, null, null, null, 2, 3, 0, 0, 1, 1, 0, 3, null, null,
    //        null, null, null, null, null, null, null, null, 3, 0, 3, 3, null, null, 0, 0, 1, 0,
    //        null, null, null, null, null, null, null, null]
    let root = tree_from_vec(vec![
        Some(3), Some(2), Some(2), Some(2), Some(2), Some(3), Some(2), Some(0), Some(0), Some(0),
        Some(1), Some(1), Some(2), Some(1), Some(1), None, None, None, None, None, None, None,
        None, None, None, Some(3), Some(2), None, None, None, None, Some(2), Some(3), Some(0),
        Some(0), Some(1), Some(1), Some(0), Some(3), None, None, None, None, None, None, None,
        None, None, None, Some(3), Some(0), Some(3), Some(3), None, None, Some(0), Some(0),
        Some(1), Some(0), None, None, None, None, None, None, None, None,
    ]);
    assert_eq!(Solution::evaluate_tree(root), true);
}
