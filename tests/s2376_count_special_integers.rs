// Tests for Problem 2376: Count Special Integers
// Java reference: src/test/java/g2301_2400/s2376_count_special_integers/SolutionTest.java

use leetcode_in_rust::s2376::count_special_integers::Solution;

#[test]
fn test_count_special_numbers() {
    assert_eq!(Solution::count_special_numbers(20), 19);
}

#[test]
fn test_count_special_numbers2() {
    assert_eq!(Solution::count_special_numbers(5), 5);
}

#[test]
fn test_count_special_numbers3() {
    assert_eq!(Solution::count_special_numbers(135), 110);
}
