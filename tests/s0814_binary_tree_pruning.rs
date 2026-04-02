// Tests for Problem 0814: Binary Tree Pruning
// Java reference: src/test/java/g0801_0900/s0814_binary_tree_pruning/SolutionTest.java

use leetcode_in_rust::s0814::binary_tree_pruning::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_prune_tree() {
    // [1, null, 0, 0, 1]
    let tree = tree_from_vec(vec![Some(1), None, Some(0), Some(0), Some(1)]);
    let result = Solution::prune_tree(tree);
    let result_vec = tree_to_vec(result);
    // Expected: [1, null, 0, null, 1]
    assert_eq!(result_vec, vec![Some(1), None, Some(0), None, Some(1)]);
}

#[test]
fn test_prune_tree2() {
    // [1, 0, 1, 0, 0, 0, 1]
    let tree = tree_from_vec(vec![
        Some(1),
        Some(0),
        Some(1),
        Some(0),
        Some(0),
        Some(0),
        Some(1),
    ]);
    let result = Solution::prune_tree(tree);
    let result_vec = tree_to_vec(result);
    // Expected: [1, null, 1, null, 1]
    assert_eq!(result_vec, vec![Some(1), None, Some(1), None, Some(1)]);
}

#[test]
fn test_prune_tree3() {
    // [1, 1, 0, 1, 1, 0, 1, 0]
    let tree = tree_from_vec(vec![
        Some(1),
        Some(1),
        Some(0),
        Some(1),
        Some(1),
        Some(0),
        Some(1),
        Some(0),
    ]);
    let result = Solution::prune_tree(tree);
    let result_vec = tree_to_vec(result);
    // Expected: [1, 1, 0, 1, 1, null, 1]
    assert_eq!(
        result_vec,
        vec![Some(1), Some(1), Some(0), Some(1), Some(1), None, Some(1)]
    );
}
