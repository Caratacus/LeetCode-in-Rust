// Tests for Problem 0687: Longest Univalue Path
// Java reference: src/test/java/g0601_0700/s0687_longest_univalue_path/SolutionTest.java

use leetcode_in_rust::s0687::longest_univalue_path::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_longest_univalue_path() {
    // Tree: [5, 4, 5, 1, 1, 5]
    let root = tree_from_vec(vec![Some(5), Some(4), Some(5), Some(1), Some(1), Some(5)]);
    assert_eq!(Solution::longest_univalue_path(root), 2);
}

#[test]
fn test_longest_univalue_path2() {
    // Tree: [1, 4, 5, 4, 4, 5]
    let root = tree_from_vec(vec![Some(1), Some(4), Some(5), Some(4), Some(4), Some(5)]);
    assert_eq!(Solution::longest_univalue_path(root), 2);
}
