// Tests for Problem 3543: Maximum Weighted K Edge Path
// Java reference: src/test/java/g3501_3600/s3543_maximum_weighted_k_edge_path/SolutionTest.java

use leetcode_in_rust::s3543::maximum_weighted_k_edge_path::Solution;

#[test]
fn test_max_weight() { assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 1], vec![1, 2, 2]], 2, 4), 3); }

#[test]
fn test_max_weight2() { assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 2], vec![0, 2, 3]], 1, 3), 2); }

#[test]
fn test_max_weight3() { assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 6], vec![1, 2, 8]], 1, 6), -1); }

#[test]
fn test_max_weight4() { assert_eq!(Solution::max_weight(3, vec![vec![0, 1, 6], vec![1, 2, 8]], 0, 6), 0); }

#[test]
fn test_max_weight5() {
    assert_eq!(Solution::max_weight(6, vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 3, 2], vec![2, 3, 5], vec![3, 4, 5], vec![3, 5, 3]], 3, 12), 11);
}

#[test]
fn test_max_weight6() {
    assert_eq!(Solution::max_weight(5, vec![vec![0, 1, 2], vec![0, 2, 3], vec![1, 3, 3], vec![2, 3, 1], vec![3, 4, 2]], 3, 7), 6);
}
