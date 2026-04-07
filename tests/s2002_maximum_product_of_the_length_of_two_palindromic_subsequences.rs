// Tests for Problem 2002: Maximum Product of the Length of Two Palindromic Subsequences
// Java reference: src/test/java/g2001_2100/s2002_maximum_product_of_the_length_of_two_palindromic_subsequences/SolutionTest.java

use leetcode_in_rust::s2002::maximum_product_of_the_length_of_two_palindromic_subsequences::Solution;

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product(String::from("leetcodecom")), 9);
}

#[test]
fn test_max_product2() {
    assert_eq!(Solution::max_product(String::from("bb")), 1);
}

#[test]
fn test_max_product3() {
    assert_eq!(Solution::max_product(String::from("accbcaxxcxx")), 25);
}
