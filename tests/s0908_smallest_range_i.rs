// Tests for Problem 0908: Smallest Range I
// Java reference: src/test/java/g0901_1000/s0908_smallest_range_i/SolutionTest.java

use leetcode_in_rust::s0908::smallest_range_i::Solution;

#[test]
fn test_smallest_range_i() {
    assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
}

#[test]
fn test_smallest_range_i2() {
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
}

#[test]
fn test_smallest_range_i3() {
    assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
}
