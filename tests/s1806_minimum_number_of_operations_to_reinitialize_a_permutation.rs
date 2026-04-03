// Tests for Problem 1806: Minimum Number of Operations to Reinitialize a Permutation
// Java reference: src/test/java/g1801_1900/s1806_minimum_number_of_operations_to_reinitialize_a_permutation/SolutionTest.java

use leetcode_in_rust::s1806::minimum_number_of_operations_to_reinitialize_a_permutation::Solution;

#[test]
fn test_reinitialize_permutation() {
    assert_eq!(Solution::reinitialize_permutation(2), 1);
}

#[test]
fn test_reinitialize_permutation2() {
    assert_eq!(Solution::reinitialize_permutation(4), 2);
}
