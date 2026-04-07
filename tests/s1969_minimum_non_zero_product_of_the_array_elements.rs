// Tests for Problem 1969: Minimum Non-Zero Product of the Array Elements
// Java reference: src/test/java/g1901_2000/s1969_minimum_non_zero_product_of_the_array_elements/SolutionTest.java

use leetcode_in_rust::s1969::minimum_non_zero_product_of_the_array_elements::Solution;

#[test]
fn test_min_non_zero_product() {
    assert_eq!(Solution::min_non_zero_product(2), 6);
}

#[test]
fn test_min_non_zero_product2() {
    assert_eq!(Solution::min_non_zero_product(3), 1512);
}
