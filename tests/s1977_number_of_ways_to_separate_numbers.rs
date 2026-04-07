// Tests for Problem 1977: Number of Ways to Separate Numbers
// Java reference: src/test/java/g1901_2000/s1977_number_of_ways_to_separate_numbers/SolutionTest.java

use leetcode_in_rust::s1977::number_of_ways_to_separate_numbers::Solution;

#[test]
fn test_number_of_combinations() {
    assert_eq!(Solution::number_of_combinations(String::from("327")), 2);
}

#[test]
fn test_number_of_combinations2() {
    assert_eq!(Solution::number_of_combinations(String::from("094")), 0);
}

#[test]
fn test_number_of_combinations3() {
    assert_eq!(Solution::number_of_combinations(String::from("0")), 0);
}
