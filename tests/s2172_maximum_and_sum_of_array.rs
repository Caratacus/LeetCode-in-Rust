// Tests for Problem 2172: Maximum AND Sum of Array
// Java reference: src/test/java/g2101_2200/s2172_maximum_and_sum_of_array/SolutionTest.java

use leetcode_in_rust::s2172::maximum_and_sum_of_array::Solution;

#[test]
fn test_maximum_and_sum() {
    assert_eq!(Solution::maximum_and_sum(vec![1, 2, 3, 4, 5, 6], 3), 9);
}

#[test]
fn test_maximum_and_sum2() {
    assert_eq!(Solution::maximum_and_sum(vec![1, 3, 10, 4, 7, 1], 9), 24);
}
