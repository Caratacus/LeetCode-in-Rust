// Tests for Problem 2342: Max Sum of a Pair with Equal Sum of Digits
// Java reference: src/test/java/g2301_2400/s2342_max_sum_of_a_pair_with_equal_sum_of_digits/SolutionTest.java

use leetcode_in_rust::s2342::max_sum_of_a_pair_with_equal_sum_of_digits::Solution;

#[test]
fn test_maximum_sum() {
    assert_eq!(Solution::maximum_sum(vec![18, 43, 36, 13, 7]), 54);
}

#[test]
fn test_maximum_sum2() {
    assert_eq!(Solution::maximum_sum(vec![10, 12, 19, 14]), -1);
}
