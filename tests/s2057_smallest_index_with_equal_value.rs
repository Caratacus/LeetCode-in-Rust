// Tests for Problem 2057: Smallest Index With Equal Value
// Java reference: src/test/java/g2001_2100/s2057_smallest_index_with_equal_value/SolutionTest.java

use leetcode_in_rust::s2057::smallest_index_with_equal_value::Solution;

#[test]
fn test_smallest_equal() {
    assert_eq!(Solution::smallest_equal(vec![0, 1, 2]), 0);
}

#[test]
fn test_smallest_equal2() {
    assert_eq!(Solution::smallest_equal(vec![4, 3, 2, 1]), 2);
}

#[test]
fn test_smallest_equal3() {
    assert_eq!(Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), -1);
}
