// Tests for Problem 0238: Product of Array Except Self
// Java reference: src/test/java/g0201_0300/s0238_product_of_array_except_self/SolutionTest.java

use leetcode_in_rust::s0238::product_of_array_except_self::Solution;

#[test]
fn test_product_except_self() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}

#[test]
fn test_product_except_self2() {
    assert_eq!(Solution::product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
}
