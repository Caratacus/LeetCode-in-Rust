// Tests for Problem 1877: Minimize Maximum Pair Sum in Array
// Java reference: src/test/java/g1801_1900/s1877_minimize_maximum_pair_sum_in_array/SolutionTest.java

use leetcode_in_rust::s1877::minimize_maximum_pair_sum_in_array::Solution;

#[test]
fn test_min_pair_sum() {
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
}

#[test]
fn test_min_pair_sum2() {
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
}
