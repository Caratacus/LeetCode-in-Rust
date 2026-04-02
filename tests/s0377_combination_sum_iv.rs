// Tests for Problem 0377: Combination Sum IV
// Java reference: src/test/java/g0301_0400/s0377_combination_sum_iv/SolutionTest.java

use leetcode_in_rust::s0377::combination_sum_iv::Solution;

#[test]
fn test_combination_sum4() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
}

#[test]
fn test_combination_sum4_2() {
    assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
}
