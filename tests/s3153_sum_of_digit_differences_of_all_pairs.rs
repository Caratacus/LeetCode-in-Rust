// Tests for Problem 3153: Sum of Digit Differences of All Pairs
// Java reference: src/test/java/g3101_3200/s3153_sum_of_digit_differences_of_all_pairs/SolutionTest.java

use leetcode_in_rust::s3153::sum_of_digit_differences_of_all_pairs::Solution;
#[test]
fn test_sum_digit_differences() {
    assert_eq!(Solution::sum_digit_differences(vec![13, 23, 12]), 4);
}
#[test]
fn test_sum_digit_differences2() {
    assert_eq!(Solution::sum_digit_differences(vec![10, 10, 10, 10]), 0);
}
