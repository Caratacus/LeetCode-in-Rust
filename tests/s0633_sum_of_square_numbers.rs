// Tests for Problem 0633: Sum of Square Numbers
// Java reference: src/test/java/g0601_0700/s0633_sum_of_square_numbers/SolutionTest.java

use leetcode_in_rust::s0633::sum_of_square_numbers::Solution;

#[test]
fn test_judge_square_sum() {
    assert_eq!(Solution::judge_square_sum(5), true);
}

#[test]
fn test_judge_square_sum2() {
    assert_eq!(Solution::judge_square_sum(3), false);
}
