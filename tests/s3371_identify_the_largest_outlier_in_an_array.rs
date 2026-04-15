// Tests for Problem 3371: Identify the Largest Outlier in an Array
// Java reference: src/test/java/g3301_3400/s3371_identify_the_largest_outlier_in_an_array/SolutionTest.java

use leetcode_in_rust::s3371::identify_the_largest_outlier_in_an_array::Solution;

#[test]
fn test_get_largest_outlier() {
    assert_eq!(Solution::get_largest_outlier(vec![2, 3, 5, 10]), 10);
}

#[test]
fn test_get_largest_outlier2() {
    assert_eq!(Solution::get_largest_outlier(vec![-2, -1, -3, -6, 4]), 4);
}

#[test]
fn test_get_largest_outlier3() {
    assert_eq!(Solution::get_largest_outlier(vec![1, 1, 1, 1, 1, 5, 5]), 5);
}

#[test]
fn test_get_largest_outlier4() {
    assert_eq!(Solution::get_largest_outlier(vec![-108, -108, -517]), -517);
}
