// Tests for Problem 0897: Increasing Order Search Tree
// Java reference: src/test/java/g0801_0900/s0897_increasing_order_search_tree/SolutionTest.java

use leetcode_in_rust::s0897::increasing_order_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_increasing_bst() {
    let root = tree_from_vec(vec![
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        Some(8),
        Some(1),
        None,
        None,
        None,
        Some(7),
        Some(9),
    ]);
    let result = Solution::increasing_bst(root);
    let result_vec = tree_to_vec(result);
    // The result should be a right-skewed tree with values in order: 1,2,3,4,5,6,7,8,9
    assert_eq!(result_vec.len(), 9);
}

#[test]
fn test_increasing_bst2() {
    let root = tree_from_vec(vec![Some(5), Some(1), Some(7)]);
    let result = Solution::increasing_bst(root);
    let result_vec = tree_to_vec(result);
    // The result should be a right-skewed tree with values in order: 1,5,7
    assert_eq!(result_vec.len(), 3);
}
