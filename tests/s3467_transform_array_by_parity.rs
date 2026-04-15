// Tests for Problem 3467: Transform Array by Parity
// Java reference: src/test/java/g3401_3500/s3467_transform_array_by_parity/SolutionTest.java

use leetcode_in_rust::s3467::transform_array_by_parity::Solution;

#[test]
fn test_transform_array() {
    assert_eq!(Solution::transform_array(vec![4, 3, 2, 1]), vec![0, 0, 1, 1]);
}

#[test]
fn test_transform_array2() {
    assert_eq!(Solution::transform_array(vec![1, 5, 1, 4, 2]), vec![0, 0, 1, 1, 1]);
}
