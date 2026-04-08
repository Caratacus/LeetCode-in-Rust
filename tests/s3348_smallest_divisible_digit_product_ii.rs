// Tests for Problem 3348: Smallest Divisible Digit Product II
// Java reference: src/test/java/g3301_3400/s3348_smallest_divisible_digit_product_ii/SolutionTest.java

use leetcode_in_rust::s3348::smallest_divisible_digit_product_ii::Solution;

#[test]
fn test_smallest_number() {
    assert_eq!(Solution::smallest_number("1234".to_string(), 256), "1488");
}

#[test]
fn test_smallest_number2() {
    assert_eq!(Solution::smallest_number("12355".to_string(), 50), "12355");
}

#[test]
fn test_smallest_number3() {
    assert_eq!(Solution::smallest_number("11111".to_string(), 26), "-1");
}

#[test]
fn test_smallest_number4() {
    assert_eq!(Solution::smallest_number("10".to_string(), 320), "588");
}

#[test]
fn test_smallest_number5() {
    assert_eq!(Solution::smallest_number("19".to_string(), 2), "21");
}
