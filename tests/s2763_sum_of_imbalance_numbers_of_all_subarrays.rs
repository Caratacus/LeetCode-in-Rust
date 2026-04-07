// Tests for Problem 2763: Sum of Imbalance Numbers of All Subarrays
// Java reference: src/test/java/g2701_2800/s2763_sum_of_imbalance_numbers_of_all_subarrays/SolutionTest.java

use leetcode_in_rust::s2763::sum_of_imbalance_numbers_of_all_subarrays::Solution;

#[test]
fn test_sum_imbalance_numbers() {
    assert_eq!(Solution::sum_imbalance_numbers(vec![2, 3, 1, 4]), 3);
}

#[test]
fn test_sum_imbalance_numbers2() {
    assert_eq!(Solution::sum_imbalance_numbers(vec![1, 3, 3, 3, 5]), 8);
}
