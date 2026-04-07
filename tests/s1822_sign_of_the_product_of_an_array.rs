// Tests for Problem 1822: Sign of the Product of an Array
// Java reference: src/test/java/g1801_1900/s1822_sign_of_the_product_of_an_array/SolutionTest.java

use leetcode_in_rust::s1822::sign_of_the_product_of_an_array::Solution;

#[test]
fn test_array_sign() {
    assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
}

#[test]
fn test_array_sign2() {
    assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
}

#[test]
fn test_array_sign3() {
    assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
}
