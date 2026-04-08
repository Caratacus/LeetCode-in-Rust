// Tests for Problem 3134: Find the Median of the Uniqueness Array
// Java reference: src/test/java/g3101_3200/s3134_find_the_median_of_the_uniqueness_array/SolutionTest.java

use leetcode_in_rust::s3134::find_the_median_of_the_uniqueness_array::Solution;
#[test]
fn test_median_of_uniqueness_array() {
    assert_eq!(Solution::median_of_uniqueness_array(vec![1, 2, 3]), 1);
}
#[test]
fn test_median_of_uniqueness_array2() {
    assert_eq!(Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
}
#[test]
fn test_median_of_uniqueness_array3() {
    assert_eq!(Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
}
