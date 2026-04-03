// Tests for Problem 0865: Smallest Subtree with all the Deepest Nodes
// Java reference: src/test/java/g0801_0900/s0865_smallest_subtree_with_all_the_deepest_nodes/SolutionTest.java

use leetcode_in_rust::s0865::smallest_subtree_with_all_the_deepest_nodes::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_subtree_with_all_deepest() {
    let root = tree_from_vec(vec![
        Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8),
        None, None, Some(7), Some(4)
    ]);
    let result = Solution::subtree_with_all_deepest(root);
    assert!(result.is_some());
    // The result should be a subtree rooted at 2 with children 7 and 4
    let vals = tree_to_vec(result);
    assert_eq!(vals, vec![Some(2), Some(7), Some(4)]);
}

#[test]
fn test_subtree_with_all_deepest2() {
    let root = tree_from_vec(vec![Some(1)]);
    let result = Solution::subtree_with_all_deepest(root);
    assert!(result.is_some());
    // The result should be the single node 1
    let vals = tree_to_vec(result);
    assert_eq!(vals, vec![Some(1)]);
}

#[test]
fn test_subtree_with_all_deepest3() {
    let root = tree_from_vec(vec![Some(0), Some(1), Some(3), None, Some(2)]);
    let result = Solution::subtree_with_all_deepest(root);
    assert!(result.is_some());
    // The result should be the single node 2
    let vals = tree_to_vec(result);
    assert_eq!(vals, vec![Some(2)]);
}
