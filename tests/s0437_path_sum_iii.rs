// Tests for Problem 0437: Path Sum III
// Java reference: src/test/java/g0401_0500/s0437_path_sum_iii/SolutionTest.java

use leetcode_in_rust::s0437::path_sum_iii::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_path_sum() {
    let root = tree_from_vec(vec![Some(10), Some(5), Some(-3), Some(3), Some(2), None, Some(11), Some(3), Some(-2), None, Some(1)]);
    assert_eq!(Solution::path_sum(root, 8), 3);
}

#[test]
fn test_path_sum2() {
    let root = tree_from_vec(vec![Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)]);
    assert_eq!(Solution::path_sum(root, 22), 3);
}
