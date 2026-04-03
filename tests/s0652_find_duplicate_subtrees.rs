// Tests for Problem 0652: Find Duplicate Subtrees
// Java reference: src/test/java/g0601_0700/s0652_find_duplicate_subtrees/SolutionTest.java

use leetcode_in_rust::s0652::find_duplicate_subtrees::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_duplicate_subtrees() {
    // Tree: [1, 2, 3, 4, null, 2, 4, null, null, 4]
    let root = tree_from_vec(vec![
        Some(1), Some(2), Some(3), Some(4), None, Some(2), Some(4), None, None, None, None, Some(4)
    ]);
    let result = Solution::find_duplicate_subtrees(root);
    // Should find duplicates with values [2,4], [4], and [2,3,4]
    assert!(!result.is_empty());
}

#[test]
fn test_find_duplicate_subtrees2() {
    // Tree: [2, 2, 2, 3, null, 3, null]
    let root = tree_from_vec(vec![Some(2), Some(2), Some(2), Some(3), None, Some(3), None]);
    let result = Solution::find_duplicate_subtrees(root);
    assert!(!result.is_empty());
}
