// Tests for Problem 3345: Smallest Divisible Digit Product I
// Java reference: src/test/java/g3301_3400/s3345_smallest_divisible_digit_product_i/SolutionTest.java

use leetcode_in_rust::s3345::smallest_divisible_digit_product_i::Solution;

#[test]
fn test_smallest_number() {
    assert_eq!(Solution::smallest_number(10, 2), 10);
}

#[test]
fn test_smallest_number2() {
    assert_eq!(Solution::smallest_number(15, 3), 16);
}
