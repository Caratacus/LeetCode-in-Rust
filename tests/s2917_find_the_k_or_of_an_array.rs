// Tests for Problem 2917: Find the K-or of an Array
// Java reference: src/test/java/g2901_3000/s2917_find_the_k_or_of_an_array/SolutionTest.java

use leetcode_in_rust::s2917::find_the_k_or_of_an_array::Solution;

#[test]
fn test_find_k_or() {
    assert_eq!(Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
}

#[test]
fn test_find_k_or2() {
    assert_eq!(Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
}

#[test]
fn test_find_k_or3() {
    assert_eq!(Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
}
