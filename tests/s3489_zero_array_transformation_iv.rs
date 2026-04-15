// Tests for Problem 3489: Zero Array Transformation IV
// Java reference: src/test/java/g3401_3500/s3489_zero_array_transformation_iv/SolutionTest.java

use leetcode_in_rust::s3489::zero_array_transformation_iv::Solution;

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

#[test]
fn test_min_zero_array3() {
    assert_eq!(
        Solution::min_zero_array(
            vec![1, 2, 3, 2, 1],
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 1], vec![4, 4, 1]]
        ),
        4
    );
}

#[test]
fn test_min_zero_array4() {
    assert_eq!(
        Solution::min_zero_array(
            vec![1, 2, 3, 2, 6],
            vec![vec![0, 1, 1], vec![0, 2, 1], vec![1, 4, 2], vec![4, 4, 4], vec![3, 4, 1], vec![4, 4, 5]]
        ),
        4
    );
}
