// Tests for Problem 2769: Find the Maximum Achievable Number
// Java reference: src/test/java/g2701_2800/s2769_find_the_maximum_achievable_number/SolutionTest.java

use leetcode_in_rust::s2769::find_the_maximum_achievable_number::Solution;

#[test]
fn test_the_maximum_achievable_x() {
    assert_eq!(Solution::the_maximum_achievable_x(4, 1), 6);
}

#[test]
fn test_the_maximum_achievable_x2() {
    assert_eq!(Solution::the_maximum_achievable_x(3, 2), 7);
}
