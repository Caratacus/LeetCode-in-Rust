// Tests for Problem 2233: Maximum Product After K Increments
// Java reference: src/test/java/g2201_2300/s2233_maximum_product_after_k_increments/SolutionTest.java

use leetcode_in_rust::s2233::maximum_product_after_k_increments::Solution;

#[test]
fn test_maximum_product() {
    assert_eq!(Solution::maximum_product(vec![0, 4], 5), 20);
}

#[test]
fn test_maximum_product2() {
    assert_eq!(Solution::maximum_product(vec![6, 3, 3, 2], 2), 216);
}
