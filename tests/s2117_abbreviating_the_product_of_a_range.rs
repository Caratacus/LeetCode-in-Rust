// Tests for Problem 2117: Abbreviating the Product of a Range
// Java reference: src/test/java/g2101_2200/s2117_abbreviating_the_product_of_a_range/SolutionTest.java

use leetcode_in_rust::s2117::abbreviating_the_product_of_a_range::Solution;

#[test]
fn test_abbreviate_product() {
    assert_eq!(Solution::abbreviate_product(1, 4), "24e0");
}

#[test]
fn test_abbreviate_product2() {
    assert_eq!(Solution::abbreviate_product(2, 11), "399168e2");
}

#[test]
fn test_abbreviate_product3() {
    assert_eq!(Solution::abbreviate_product(371, 375), "7219856259e3");
}
