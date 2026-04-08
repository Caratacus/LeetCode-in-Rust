// Tests for Problem 3314: Construct the Minimum Bitwise Array I
// Java reference: src/test/java/g3301_3400/s3314_construct_the_minimum_bitwise_array_i/SolutionTest.java

use leetcode_in_rust::s3314::construct_the_minimum_bitwise_array_i::Solution;

#[test]
fn test_min_bitwise_array() {
    assert_eq!(Solution::min_bitwise_array(vec![2, 3, 5, 7]), vec![-1, 1, 4, 3]);
}

#[test]
fn test_min_bitwise_array2() {
    assert_eq!(Solution::min_bitwise_array(vec![11, 13, 31]), vec![9, 12, 15]);
}
