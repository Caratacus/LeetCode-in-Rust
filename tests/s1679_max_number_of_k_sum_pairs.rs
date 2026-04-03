// Tests for Problem 1679: Max Number of K-Sum Pairs
// Java reference: src/test/java/g1601_1700/s1679_max_number_of_k_sum_pairs/SolutionTest.java

use leetcode_in_rust::s1679::max_number_of_k_sum_pairs::Solution;

#[test]
fn test_max_operations() {
    assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
}

#[test]
fn test_max_operations2() {
    assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
}
