// Tests for Problem 2719: Count of Integers
// Java reference: src/test/java/g2701_2800/s2719_count_of_integers/SolutionTest.java

use leetcode_in_rust::s2719::count_of_integers::Solution;

#[test]
fn test_count() {
    assert_eq!(Solution::count("1".to_string(), "12".to_string(), 1, 8), 11);
}

#[test]
fn test_count2() {
    assert_eq!(Solution::count("1".to_string(), "5".to_string(), 1, 5), 5);
}
