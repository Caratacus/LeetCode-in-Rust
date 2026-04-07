// Tests for Problem 2016: Maximum Difference Between Increasing Elements
// Java reference: src/test/java/g2001_2100/s2016_maximum_difference_between_increasing_elements/SolutionTest.java

use leetcode_in_rust::s2016::maximum_difference_between_increasing_elements::Solution;

#[test]
fn test_maximum_difference() {
    assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
}

#[test]
fn test_maximum_difference2() {
    assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
}

#[test]
fn test_maximum_difference3() {
    assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
}
