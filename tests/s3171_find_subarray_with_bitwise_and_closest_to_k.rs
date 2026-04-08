// Tests for Problem 3171: Find Subarray with Bitwise AND Closest to K
// Java reference: src/test/java/g3101_3200/s3171_find_subarray_with_bitwise_and_closest_to_k/SolutionTest.java

use leetcode_in_rust::s3171::find_subarray_with_bitwise_and_closest_to_k::Solution;

#[test]
fn test_minimum_difference() {
    assert_eq!(Solution::minimum_difference(vec![1, 2, 4, 5], 3), 1);
}
#[test]
fn test_minimum_difference2() {
    assert_eq!(Solution::minimum_difference(vec![1, 2, 1, 2], 2), 0);
}
#[test]
fn test_minimum_difference3() {
    assert_eq!(Solution::minimum_difference(vec![1], 10), 9);
}