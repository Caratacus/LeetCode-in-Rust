// Tests for Problem 2827: Number of Beautiful Integers in the Range
// Java reference: src/test/java/g2801_2900/s2827_number_of_beautiful_integers_in_the_range/SolutionTest.java

use leetcode_in_rust::s2827::number_of_beautiful_integers_in_the_range::Solution;

#[test]
fn test_number_of_beautiful_integers() {
    assert_eq!(Solution::number_of_beautiful_integers(10, 20, 3), 2);
}

#[test]
fn test_number_of_beautiful_integers2() {
    assert_eq!(Solution::number_of_beautiful_integers(1, 10, 1), 1);
}

#[test]
fn test_number_of_beautiful_integers3() {
    assert_eq!(Solution::number_of_beautiful_integers(5, 5, 2), 0);
}
