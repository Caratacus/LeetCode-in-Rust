// Tests for Problem 2815: Max Pair Sum in an Array
// Java reference: src/test/java/g2801_2900/s2815_max_pair_sum_in_an_array/SolutionTest.java

use leetcode_in_rust::s2815::max_pair_sum_in_an_array::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![51, 71, 17, 24, 42]), 88);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![1, 2, 3, 4]), -1);
}
