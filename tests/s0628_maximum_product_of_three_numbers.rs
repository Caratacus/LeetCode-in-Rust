// Tests for Problem 0628: Maximum Product of Three Numbers
// Java reference: src/test/java/g0601_0700/s0628_maximum_product_of_three_numbers/SolutionTest.java

use leetcode_in_rust::s0628::maximum_product_of_three_numbers::Solution;

#[test]
fn test_maximum_product() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
}

#[test]
fn test_maximum_product2() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
}

#[test]
fn test_maximum_product3() {
    assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
}
