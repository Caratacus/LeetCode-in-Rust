// Tests for Problem 3137: Minimum Number of Operations to Make Word K-Periodic
// Java reference: src/test/java/g3101_3200/s3137_minimum_number_of_operations_to_make_word_k_periodic/SolutionTest.java

use leetcode_in_rust::s3137::minimum_number_of_operations_to_make_word_k_periodic::Solution;
#[test]
fn test_minimum_operations_to_make_k_periodic() {
    assert_eq!(Solution::minimum_operations_to_make_k_periodic(String::from("leetcodeleet"), 4), 1);
}
#[test]
fn test_minimum_operations_to_make_k_periodic2() {
    assert_eq!(Solution::minimum_operations_to_make_k_periodic(String::from("leetcoleet"), 2), 3);
}
