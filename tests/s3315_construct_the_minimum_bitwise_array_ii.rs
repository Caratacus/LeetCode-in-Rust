// Tests for Problem 3315: Construct the Minimum Bitwise Array II
// Java reference: src/test/java/g3301_3400/s3315_construct_the_minimum_bitwise_array_ii/SolutionTest.java

use leetcode_in_rust::s3315::construct_the_minimum_bitwise_array_ii::Solution;

#[test]
fn test_min_bitwise_array() {
    assert_eq!(Solution::min_bitwise_array(vec![2, 3, 5, 7]), vec![-1, 1, 4, 3]);
}

#[test]
fn test_min_bitwise_array2() {
    assert_eq!(Solution::min_bitwise_array(vec![11, 13, 31]), vec![9, 12, 15]);
}
