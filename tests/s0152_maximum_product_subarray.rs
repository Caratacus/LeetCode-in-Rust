// Tests for Problem 0152: Maximum Product Subarray
// Java reference: src/test/java/g0121_0200/s0152_maximum_product_subarray/SolutionTest.java

use leetcode_in_rust::s0152::maximum_product_subarray::Solution;

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
}

#[test]
fn test_max_product2() {
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
}

#[test]
fn test_max_product3() {
    assert_eq!(Solution::max_product(vec![-2]), -2);
}
