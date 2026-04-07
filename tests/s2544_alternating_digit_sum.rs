// Tests for Problem 2544: Alternating Digit Sum
// Java reference: src/test/java/g2501_2600/s2544_alternating_digit_sum/SolutionTest.java
use leetcode_in_rust::s2544::alternating_digit_sum::Solution;

#[test]
fn test_alternate_digit_sum() {
    assert_eq!(Solution::alternate_digit_sum(521), 4);
}
#[test]
fn test_alternate_digit_sum2() {
    assert_eq!(Solution::alternate_digit_sum(111), 1);
}
#[test]
fn test_alternate_digit_sum3() {
    assert_eq!(Solution::alternate_digit_sum(886996), 0);
}
