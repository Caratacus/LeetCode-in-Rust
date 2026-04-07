// Tests for Problem 2749: Minimum Operations to Make the Integer Zero
// Java reference: src/test/java/g2701_2800/s2749_minimum_operations_to_make_the_integer_zero/SolutionTest.java

use leetcode_in_rust::s2749::minimum_operations_to_make_the_integer_zero::Solution;

#[test]
fn test_make_the_integer_zero() {
    assert_eq!(Solution::make_the_integer_zero(3, -2), 3);
}

#[test]
fn test_make_the_integer_zero2() {
    assert_eq!(Solution::make_the_integer_zero(5, 7), -1);
}
