// Tests for Problem 3117: Minimum Sum of Values by Dividing Array
// Java reference: src/test/java/g3101_3200/s3117_minimum_sum_of_values_by_dividing_array/SolutionTest.java

use leetcode_in_rust::s3117::minimum_sum_of_values_by_dividing_array::Solution;

#[test]
fn test_minimum_value_sum() {
    assert_eq!(
        Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]),
        12
    );
}

#[test]
fn test_minimum_value_sum2() {
    assert_eq!(
        Solution::minimum_value_sum(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5]),
        17
    );
}

#[test]
fn test_minimum_value_sum3() {
    assert_eq!(Solution::minimum_value_sum(vec![1, 2, 3, 4], vec![2]), -1);
}
