// Tests for Problem 1960: Maximum Product of the Length of Two Palindromic Substrings
// Java reference: src/test/java/g1901_2000/s1960_maximum_product_of_the_length_of_two_palindromic_substrings/SolutionTest.java

use leetcode_in_rust::s1960::maximum_product_of_the_length_of_two_palindromic_substrings::Solution;

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product("ababbb".to_string()), 9);
}

#[test]
fn test_max_product2() {
    assert_eq!(Solution::max_product("zaaaxbbby".to_string()), 9);
}
