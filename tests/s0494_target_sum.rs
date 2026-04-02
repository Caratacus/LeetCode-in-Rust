// Tests for Problem 0494: Target Sum
// Java reference: src/test/java/g0401_0500/s0494_target_sum/SolutionTest.java

use leetcode_in_rust::s0494::target_sum::Solution;

#[test]
fn test_find_target_sum_ways() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
}

#[test]
fn test_find_target_sum_ways2() {
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
}
