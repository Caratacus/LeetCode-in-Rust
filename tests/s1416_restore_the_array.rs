// Tests for Problem 1416: Restore the Array
// Java reference: src/test/java/g1401_1500/s1416_restore_the_array/SolutionTest.java

use leetcode_in_rust::s1416::restore_the_array::Solution;

#[test]
fn test_number_of_arrays() {
    assert_eq!(Solution::number_of_arrays("1000".to_string(), 10000), 1);
}

#[test]
fn test_number_of_arrays2() {
    assert_eq!(Solution::number_of_arrays("1000".to_string(), 10), 0);
}

#[test]
fn test_number_of_arrays3() {
    assert_eq!(Solution::number_of_arrays("1317".to_string(), 2000), 8);
}
