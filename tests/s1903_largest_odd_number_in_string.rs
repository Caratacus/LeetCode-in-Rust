// Tests for Problem 1903: Largest Odd Number in String
// Java reference: src/test/java/g1901_2000/s1903_largest_odd_number_in_string/SolutionTest.java

use leetcode_in_rust::s1903::largest_odd_number_in_string::Solution;

#[test]
fn test_largest_odd_number() {
    assert_eq!(Solution::largest_odd_number("52".to_string()), "5".to_string());
}

#[test]
fn test_largest_odd_number2() {
    assert_eq!(Solution::largest_odd_number("4206".to_string()), "".to_string());
}

#[test]
fn test_largest_odd_number3() {
    assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427".to_string());
}
