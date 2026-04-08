// Tests for Problem 3300: Minimum Element After Replacement With Digit Sum
// Java reference: src/test/java/g3201_3300/s3300_minimum_element_after_replacement_with_digit_sum/SolutionTest.java

use leetcode_in_rust::s3300::minimum_element_after_replacement_with_digit_sum::Solution;

#[test]
fn test_min_element() {
    assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
}

#[test]
fn test_min_element2() {
    assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
}

#[test]
fn test_min_element3() {
    assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
}
