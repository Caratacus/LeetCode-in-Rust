// Tests for Problem 0113: Path Sum II
// Java reference: src/test/java/g0101_0200/s0113_path_sum_ii/SolutionTest.java

use leetcode_in_rust::s0113::path_sum_ii::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_path_sum() {
    let root = tree_from_vec(vec![Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)]);
    let result = Solution::path_sum(root, 22);
    assert_eq!(result, vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]);
}

#[test]
fn test_path_sum2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    let result = Solution::path_sum(root, 5);
    assert_eq!(result, vec![] as Vec<Vec<i32>>);
}

#[test]
fn test_path_sum3() {
    let root = tree_from_vec(vec![Some(1), Some(2)]);
    let result = Solution::path_sum(root, 1);
    assert_eq!(result, vec![] as Vec<Vec<i32>>);
}
