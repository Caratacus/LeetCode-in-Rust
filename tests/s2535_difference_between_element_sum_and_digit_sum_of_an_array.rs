// Tests for Problem 2535: Difference Between Element Sum and Digit Sum of an Array
// Java reference: src/test/java/g2501_2600/s2535_difference_between_element_sum_and_digit_sum_of_an_array/SolutionTest.java

use leetcode_in_rust::s2535::difference_between_element_sum_and_digit_sum_of_an_array::Solution;

#[test]
fn test_difference_of_sum() {
    assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
}
#[test]
fn test_difference_of_sum2() {
    assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
}
