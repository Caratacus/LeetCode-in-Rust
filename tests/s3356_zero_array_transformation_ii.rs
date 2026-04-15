// Tests for Problem 3356: Zero Array Transformation II
// Java reference: src/test/java/g3301_3400/s3356_zero_array_transformation_ii/SolutionTest.java

use leetcode_in_rust::s3356::zero_array_transformation_ii::Solution;

#[test]
fn test_min_zero_array() {
    assert_eq!(
        Solution::min_zero_array(vec![2, 0, 2], vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]),
        2
    );
}

#[test]
fn test_min_zero_array2() {
    assert_eq!(
        Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
        -1
    );
}
