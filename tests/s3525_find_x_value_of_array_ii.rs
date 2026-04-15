// Tests for Problem 3525: Find X Value of Array II
// Java reference: src/test/java/g3501_3600/s3525_find_x_value_of_array_ii/SolutionTest.java

use leetcode_in_rust::s3525::find_x_value_of_array_ii::Solution;

#[test]
fn test_result_array() {
    assert_eq!(
        Solution::result_array(vec![1, 2, 3, 4, 5], 3, vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]]),
        vec![2, 2, 2]
    );
}

#[test]
fn test_result_array2() {
    assert_eq!(
        Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4, vec![vec![0, 2, 0, 2], vec![0, 2, 0, 1]]),
        vec![1, 0]
    );
}

#[test]
fn test_result_array3() {
    assert_eq!(
        Solution::result_array(vec![1, 1, 2, 1, 1], 2, vec![vec![2, 1, 0, 1]]),
        vec![5]
    );
}

#[test]
fn test_result_array4() {
    assert_eq!(
        Solution::result_array(vec![9, 10, 7], 1, vec![vec![0, 8, 1, 0]]),
        vec![2]
    );
}
