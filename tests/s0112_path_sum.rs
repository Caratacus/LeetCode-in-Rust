// Tests for Problem 0112: Path Sum
// Java reference: src/test/java/g0101_0200/s0112_path_sum/SolutionTest.java

use leetcode_in_rust::s0112::path_sum::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_has_path_sum() {
    let root = tree_from_vec(vec![Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)]);
    assert_eq!(Solution::has_path_sum(root, 22), true);
}

#[test]
fn test_has_path_sum2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::has_path_sum(root, 5), false);
}

#[test]
fn test_has_path_sum3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::has_path_sum(root, 0), false);
}
