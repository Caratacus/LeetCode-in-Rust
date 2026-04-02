// Tests for Problem 1281: Subtract the Product and Sum of Digits of an Integer
// Java reference: src/test/java/g1201_1300/s1281_subtract_the_product_and_sum_of_digits_of_an_integer/SolutionTest.java

use leetcode_in_rust::s1281::subtract_the_product_and_sum_of_digits_of_an_integer::Solution;

#[test]
fn test_subtract_product_and_sum() {
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
}

#[test]
fn test_subtract_product_and_sum2() {
    assert_eq!(Solution::subtract_product_and_sum(4421), 21);
}
