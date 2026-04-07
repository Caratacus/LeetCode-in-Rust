// Tests for Problem 2165: Smallest Value of the Rearranged Number
// Java reference: src/test/java/g2101_2200/s2165_smallest_value_of_the_rearranged_number/SolutionTest.java

use leetcode_in_rust::s2165::smallest_value_of_the_rearranged_number::Solution;

#[test]
fn test_smallest_number() {
    assert_eq!(Solution::smallest_number(310), 103);
}

#[test]
fn test_smallest_number2() {
    assert_eq!(Solution::smallest_number(-7605), -7650);
}
