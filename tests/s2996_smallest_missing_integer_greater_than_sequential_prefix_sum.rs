// Tests for Problem 2996: Smallest Missing Integer Greater Than Sequential Prefix Sum
// Java reference: src/test/java/g2901_3000/s2996_smallest_missing_integer_greater_than_sequential_prefix_sum/SolutionTest.java

use leetcode_in_rust::s2996::smallest_missing_integer_greater_than_sequential_prefix_sum::Solution;

#[test]
fn test_missing_integer() {
    assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
}

#[test]
fn test_missing_integer2() {
    assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
}
