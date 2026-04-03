// Tests for Problem 1458: Max Dot Product of Two Subsequences
// Java reference: src/test/java/g1401_1500/s1458_max_dot_product_of_two_subsequences/SolutionTest.java

use leetcode_in_rust::s1458::max_dot_product_of_two_subsequences::Solution;

#[test]
fn test_max_dot_product() {
    assert_eq!(Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]), 18);
}

#[test]
fn test_max_dot_product2() {
    assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
}

#[test]
fn test_max_dot_product3() {
    assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
}
