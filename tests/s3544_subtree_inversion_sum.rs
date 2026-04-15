// Tests for Problem 3544: Subtree Inversion Sum
// Java reference: src/test/java/g3501_3600/s3544_subtree_inversion_sum/SolutionTest.java

use leetcode_in_rust::s3544::subtree_inversion_sum::Solution;

#[test]
fn test_subtree_inversion_sum() {
    assert_eq!(Solution::subtree_inversion_sum(vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]], vec![4, -8, -6, 3, 7, -2, 5], 2), 27i64);
}

#[test]
fn test_subtree_inversion_sum2() {
    assert_eq!(Solution::subtree_inversion_sum(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![-1, 3, -2, 4, -5], 2), 9i64);
}

#[test]
fn test_subtree_inversion_sum3() {
    assert_eq!(Solution::subtree_inversion_sum(vec![vec![0, 1], vec![0, 2]], vec![0, -1, -2], 3), 3i64);
}
