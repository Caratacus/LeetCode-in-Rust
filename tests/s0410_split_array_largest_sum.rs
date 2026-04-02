// Tests for Problem 0410: Split Array Largest Sum
// Java reference: src/test/java/g0401_0500/s0410_split_array_largest_sum/SolutionTest.java

use leetcode_in_rust::s0410::split_array_largest_sum::Solution;

#[test]
fn test_split_array() {
    assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
}

#[test]
fn test_split_array2() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
}

#[test]
fn test_split_array3() {
    assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
}
