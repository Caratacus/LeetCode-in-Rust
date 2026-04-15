// Tests for Problem 3524: Find X Value of Array I
// Java reference: src/test/java/g3501_3600/s3524_find_x_value_of_array_i/SolutionTest.java

use leetcode_in_rust::s3524::find_x_value_of_array_i::Solution;

#[test]
fn test_result_array() {
    assert_eq!(Solution::result_array(vec![1, 2, 3, 4, 5], 3), vec![9i64, 2, 4]);
}

#[test]
fn test_result_array2() {
    assert_eq!(Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4), vec![18i64, 1, 2, 0]);
}

#[test]
fn test_result_array3() {
    assert_eq!(Solution::result_array(vec![1, 1, 2, 1, 1], 2), vec![9i64, 6]);
}
