// Tests for Problem 1464: Maximum Product of Two Elements in an Array
// Java reference: src/test/java/g1401_1500/s1464_maximum_product_of_two_elements_in_an_array/SolutionTest.java

use leetcode_in_rust::s1464::maximum_product_of_two_elements_in_an_array::Solution;

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
}

#[test]
fn test_max_product2() {
    assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
}

#[test]
fn test_max_product3() {
    assert_eq!(Solution::max_product(vec![3, 7]), 12);
}
