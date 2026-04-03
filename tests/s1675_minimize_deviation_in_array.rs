// Tests for Problem 1675: Minimize Deviation in Array
// Java reference: src/test/java/g1601_1700/s1675_minimize_deviation_in_array/SolutionTest.java

use leetcode_in_rust::s1675::minimize_deviation_in_array::Solution;

#[test]
fn test_minimum_deviation() {
    assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
}

#[test]
fn test_minimum_deviation2() {
    assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
}

#[test]
fn test_minimum_deviation3() {
    assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
}
