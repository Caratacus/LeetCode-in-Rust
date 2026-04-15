// Tests for Problem 3355: Zero Array Transformation I
// Java reference: src/test/java/g3301_3400/s3355_zero_array_transformation_i/SolutionTest.java

use leetcode_in_rust::s3355::zero_array_transformation_i::Solution;

#[test]
fn test_is_zero_array() {
    assert_eq!(Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]), true);
}

#[test]
fn test_is_zero_array2() {
    assert_eq!(
        Solution::is_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3], vec![0, 2]]),
        false
    );
}

#[test]
fn test_is_zero_array3() {
    assert_eq!(
        Solution::is_zero_array(vec![-1, 0, 1], vec![vec![1, 3], vec![0, 2]]),
        true
    );
}
