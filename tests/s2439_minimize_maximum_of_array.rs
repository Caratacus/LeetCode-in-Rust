// Tests for Problem 2439: Minimize Maximum of Array
// Java reference: src/test/java/g2401_2500/s2439_minimize_maximum_of_array/SolutionTest.java

use leetcode_in_rust::s2439::minimize_maximum_of_array::Solution;

#[test]
fn test_minimize_array_value() {
    assert_eq!(Solution::minimize_array_value(vec![3, 7, 1, 6]), 5);
}

#[test]
fn test_minimize_array_value2() {
    assert_eq!(Solution::minimize_array_value(vec![10, 1]), 10);
}
